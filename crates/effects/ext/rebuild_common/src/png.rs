//! 8-bit PNG preview writer: filter 0 rows, stored (uncompressed) deflate blocks, no crates. Plus a checker that re-reads
//! our own output (signature, chunk CRCs, zlib header, adler32) for tests.

use std::path::Path;

fn crc_table() -> [u32; 256] {
    let mut t = [0u32; 256];
    for n in 0..256u32 {
        let mut c = n;
        for _ in 0..8 {
            c = if c & 1 != 0 { 0xedb8_8320 ^ (c >> 1) } else { c >> 1 };
        }
        t[n as usize] = c;
    }
    t
}

/// CRC-32 (IEEE, as PNG chunks use).
pub fn crc32(data: &[u8]) -> u32 {
    let t = crc_table();
    let mut c = 0xffff_ffffu32;
    for &b in data {
        c = t[((c ^ b as u32) & 0xff) as usize] ^ (c >> 8);
    }
    c ^ 0xffff_ffff
}

/// Adler-32 of the raw (uncompressed) stream.
pub fn adler32(data: &[u8]) -> u32 {
    let (mut a, mut b) = (1u32, 0u32);
    for &x in data {
        a = (a + x as u32) % 65521;
        b = (b + a) % 65521;
    }
    (b << 16) | a
}

fn chunk(out: &mut Vec<u8>, kind: &[u8; 4], body: &[u8]) {
    out.extend_from_slice(&(body.len() as u32).to_be_bytes());
    let mut crc_in = Vec::with_capacity(4 + body.len());
    crc_in.extend_from_slice(kind);
    crc_in.extend_from_slice(body);
    out.extend_from_slice(&crc_in);
    out.extend_from_slice(&crc32(&crc_in).to_be_bytes());
}

/// Encode `channels` (1 = gray, 3 = RGB) interleaved 8-bit samples, `width * height * channels` bytes.
pub fn encode(width: u32, height: u32, channels: usize, pixels: &[u8]) -> Vec<u8> {
    assert!(channels == 1 || channels == 3, "png::encode supports 1 or 3 channels");
    let row = width as usize * channels;
    assert_eq!(pixels.len(), row * height as usize, "png::encode: pixel buffer size mismatch");
    let mut raw = Vec::with_capacity((row + 1) * height as usize);
    for y in 0..height as usize {
        raw.push(0); // filter type 0
        raw.extend_from_slice(&pixels[y * row..(y + 1) * row]);
    }
    let mut z = vec![0x78, 0x01];
    let mut blocks = raw.chunks(65535).peekable();
    if blocks.peek().is_none() {
        z.extend_from_slice(&[1, 0, 0, 0xff, 0xff]);
    }
    while let Some(b) = blocks.next() {
        z.push(if blocks.peek().is_none() { 1 } else { 0 });
        z.extend_from_slice(&(b.len() as u16).to_le_bytes());
        z.extend_from_slice(&(!(b.len() as u16)).to_le_bytes());
        z.extend_from_slice(b);
    }
    z.extend_from_slice(&adler32(&raw).to_be_bytes());
    let mut out = vec![0x89, b'P', b'N', b'G', 0x0d, 0x0a, 0x1a, 0x0a];
    let mut ihdr = Vec::new();
    ihdr.extend_from_slice(&width.to_be_bytes());
    ihdr.extend_from_slice(&height.to_be_bytes());
    ihdr.extend_from_slice(&[8, if channels == 1 { 0 } else { 2 }, 0, 0, 0]);
    chunk(&mut out, b"IHDR", &ihdr);
    chunk(&mut out, b"IDAT", &z);
    chunk(&mut out, b"IEND", &[]);
    out
}

/// Write a gray PNG file.
pub fn write_gray8(path: &Path, width: u32, height: u32, pixels: &[u8]) -> Result<(), String> {
    std::fs::write(path, encode(width, height, 1, pixels)).map_err(|e| format!("{}: {}", path.display(), e))
}

/// Write an RGB PNG file.
pub fn write_rgb8(path: &Path, width: u32, height: u32, pixels: &[u8]) -> Result<(), String> {
    std::fs::write(path, encode(width, height, 3, pixels)).map_err(|e| format!("{}: {}", path.display(), e))
}

/// Map an f32 plane to 8-bit gray: `lo` to 0, `hi` to 255, clamped; NaN and -inf map to 0, +inf to 255.
pub fn gray_from_f32(data: &[f32], width: usize, height: usize, stride: usize, lo: f32, hi: f32) -> Vec<u8> {
    let mut out = Vec::with_capacity(width * height);
    for y in 0..height {
        for x in 0..width {
            let v = data[y * stride + x];
            let t = if v.is_nan() { 0.0 } else { ((v - lo) / (hi - lo)).clamp(0.0, 1.0) };
            out.push((t * 255.0 + 0.5) as u8);
        }
    }
    out
}

/// Decoded image of our own writer: (width, height, channels, samples).
pub type Decoded = (u32, u32, usize, Vec<u8>);

/// Parse a PNG written by [`encode`]: checks the signature, every chunk CRC, the zlib header, stored blocks and adler32.
pub fn verify(png: &[u8]) -> Result<Decoded, String> {
    if png.len() < 8 || png[..8] != [0x89, b'P', b'N', b'G', 0x0d, 0x0a, 0x1a, 0x0a] {
        return Err("bad signature".into());
    }
    let mut pos = 8;
    let (mut w, mut h, mut ch) = (0u32, 0u32, 0usize);
    let mut idat = Vec::new();
    let mut ended = false;
    while pos + 12 <= png.len() {
        let len = u32::from_be_bytes([png[pos], png[pos + 1], png[pos + 2], png[pos + 3]]) as usize;
        if pos + 12 + len > png.len() {
            return Err("chunk overruns the file".into());
        }
        let kind = &png[pos + 4..pos + 8];
        let body = &png[pos + 8..pos + 8 + len];
        let want = u32::from_be_bytes([png[pos + 8 + len], png[pos + 9 + len], png[pos + 10 + len], png[pos + 11 + len]]);
        if crc32(&png[pos + 4..pos + 8 + len]) != want {
            return Err(format!("bad CRC in chunk {}", String::from_utf8_lossy(kind)));
        }
        match kind {
            b"IHDR" => {
                w = u32::from_be_bytes([body[0], body[1], body[2], body[3]]);
                h = u32::from_be_bytes([body[4], body[5], body[6], body[7]]);
                ch = match body[9] {
                    0 => 1,
                    2 => 3,
                    t => return Err(format!("unsupported colour type {}", t)),
                };
            }
            b"IDAT" => idat.extend_from_slice(body),
            b"IEND" => ended = true,
            _ => {}
        }
        pos += 12 + len;
    }
    if !ended {
        return Err("no IEND".into());
    }
    if idat.len() < 6 || idat[0] != 0x78 || ((idat[0] as u32) << 8 | idat[1] as u32) % 31 != 0 {
        return Err("bad zlib header".into());
    }
    let mut raw = Vec::new();
    let mut p = 2;
    loop {
        let hdr = *idat.get(p).ok_or("truncated deflate")?;
        if hdr & 0x06 != 0 {
            return Err("not a stored block".into());
        }
        let len = u16::from_le_bytes([idat[p + 1], idat[p + 2]]) as usize;
        let nlen = u16::from_le_bytes([idat[p + 3], idat[p + 4]]);
        if nlen != !(len as u16) {
            return Err("bad stored block length".into());
        }
        raw.extend_from_slice(&idat[p + 5..p + 5 + len]);
        p += 5 + len;
        if hdr & 1 == 1 {
            break;
        }
    }
    let ad = u32::from_be_bytes([idat[p], idat[p + 1], idat[p + 2], idat[p + 3]]);
    if adler32(&raw) != ad {
        return Err("bad adler32".into());
    }
    let row = w as usize * ch;
    if raw.len() != (row + 1) * h as usize {
        return Err("raw size mismatch".into());
    }
    let mut px = Vec::with_capacity(row * h as usize);
    for y in 0..h as usize {
        if raw[y * (row + 1)] != 0 {
            return Err("unsupported filter".into());
        }
        px.extend_from_slice(&raw[y * (row + 1) + 1..(y + 1) * (row + 1)]);
    }
    Ok((w, h, ch, px))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crc_and_adler_known_answers() {
        assert_eq!(crc32(b"123456789"), 0xcbf4_3926);
        assert_eq!(crc32(b"IEND"), 0xae42_6082);
        assert_eq!(adler32(b"Wikipedia"), 0x11e6_0398);
    }

    #[test]
    fn gray_round_trip() {
        let px: Vec<u8> = (0..(5 * 3)).map(|i| (i * 17) as u8).collect();
        let png = encode(5, 3, 1, &px);
        let (w, h, c, back) = verify(&png).unwrap();
        assert_eq!((w, h, c), (5, 3, 1));
        assert_eq!(back, px);
    }

    #[test]
    fn rgb_round_trip_over_several_deflate_blocks() {
        let (w, h) = (200u32, 120u32); // 200*3+1 per row * 120 = 72120 raw bytes: two stored blocks
        let px: Vec<u8> = (0..(w * h * 3)).map(|i| (i % 251) as u8).collect();
        let png = encode(w, h, 3, &px);
        let (w2, h2, c, back) = verify(&png).unwrap();
        assert_eq!((w2, h2, c), (w, h, 3));
        assert_eq!(back, px);
    }

    #[test]
    fn corruption_is_detected() {
        let px = vec![7u8; 16];
        let mut png = encode(4, 4, 1, &px);
        let n = png.len();
        let mut flipped_crc = png.clone();
        flipped_crc[n - 20] ^= 1; // inside IDAT adler or data, CRC must fail
        assert!(verify(&flipped_crc).is_err());
        png[0] = 0;
        assert_eq!(verify(&png).unwrap_err(), "bad signature");
    }

    #[test]
    fn gray_mapping() {
        let g = gray_from_f32(&[0.0, 0.5, 1.0, f32::NAN, f32::INFINITY, -3.0], 3, 2, 3, 0.0, 1.0);
        assert_eq!(g, vec![0, 128, 255, 0, 255, 0]);
    }

    #[test]
    fn file_writer() {
        let p = std::env::temp_dir().join("fx_common_png_test.png");
        write_gray8(&p, 2, 2, &[0, 64, 128, 255]).unwrap();
        let (w, h, _, px) = verify(&std::fs::read(&p).unwrap()).unwrap();
        assert_eq!((w, h, px), (2, 2, vec![0, 64, 128, 255]));
        std::fs::remove_file(&p).ok();
    }
}
