//! Cross-processor check: seeded draws at four NaN levels through the whole map generator; one text line per draw (FNV-1a of the output words).
//! The lines are written to the file named by ND_OUT when set; the same file from an x86_64 run and an aarch64 run must be equal.
use optical_flow_interp_maps::native::*;

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.0 >> 33
    }
    fn word(&mut self, level: u32) -> f32 {
        let r = self.next() % 16;
        let b: u32 = match (level, r) {
            (_, 0) => 0,
            (_, 1) => 0x8000_0000,
            (_, 2) => 0x7f80_0000,
            (_, 3) => 0xff80_0000,
            (_, 4) => 0x0000_0123,
            (1, 5) => 0xffc0_0000,
            (2, 5) => 0x7fc0_0000,
            (3, 5) => (self.next() as u32) | 0x7f80_0001,
            (_, 6) => 0x7f7f_ffff,
            _ => ((self.next() as f32) / (1u64 << 30) as f32 - 1.0).to_bits(),
        };
        f32::from_bits(b)
    }
}

fn fnv(words: &[f32]) -> u64 {
    let mut h = 0xcbf29ce484222325u64;
    for w in words {
        for b in w.to_bits().to_le_bytes() {
            h = (h ^ b as u64).wrapping_mul(0x100000001b3);
        }
    }
    h
}

#[test]
fn draws_print_equal_lines_on_every_processor() {
    let mut r = Rng(20261112);
    let mut out = String::new();
    for i in 0..400u32 {
        let level = i % 4;
        let (w, h) = (2 + (r.next() % 8) as usize, 2 + (r.next() % 6) as usize);
        let n = w * h;
        let nc = 1 + (r.next() % 3) as usize;
        let mk = |r: &mut Rng, k: usize| -> Vec<Vec<f32>> { (0..k).map(|_| (0..n).map(|_| r.word(level)).collect()).collect() };
        let prev = mk(&mut r, nc);
        let next = mk(&mut r, nc);
        let fl = mk(&mut r, 2);
        let scale = r.word(0);
        let res = interp_maps(w, h, &prev, &next, &fl[0], &fl[1], scale, 0.0).unwrap();
        let all: Vec<f32> = res.iter().flat_map(|p| p.iter().copied()).collect();
        out += &format!("{i}\tL{level}\t{w}x{h}\t{:016x}\n", fnv(&all));
    }
    if let Ok(p) = std::env::var("ND_OUT") {
        std::fs::write(p, out).unwrap();
    }
}
