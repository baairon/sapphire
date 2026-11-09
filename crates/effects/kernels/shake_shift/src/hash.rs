//! Hash table builder 0x5b9850 and the two hashes 0x5b9930 (one argument) and 0x5b9970 (two arguments).

pub const HASH_TABLE_LEN: usize = 256;
const LCG_ADD: u64 = 150_889;

/// One generator step, as the code computes it: `ecx = x << 12` (32-bit), `n = rcx + 0x24d69`, signed 64x64 -> 128 multiply by
/// 0x5dfc995681872319, `q = high >> 18`, result `rcx + q * -714025 + 0x24d69` (low 32 bits kept). Equals `(4096 x + 150889) mod 714025`.
#[inline]
pub fn lcg_step(x: u32) -> u32 {
    const MAGIC: i64 = 0x5dfc_9956_8187_2319_u64 as i64;
    let rcx = (x << 12) as u64;
    let n = rcx.wrapping_add(LCG_ADD);
    let prod = (n as i64 as i128) * (MAGIC as i128);
    let q = ((prod >> 64) as u64) >> 18;
    rcx.wrapping_add(q.wrapping_mul(0xffff_ffff_fff5_1ad7)).wrapping_add(LCG_ADD) as u32
}

/// The 256-dword table the builder fills, plus the generator state it stores to a second cell (`tail`).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct HashTable {
    pub t: [u32; HASH_TABLE_LEN],
    pub tail: u32,
}

impl HashTable {
    /// 0x5b9850 from the cleared guard state: entry k = `(y1 >> 2) + (y2 << 8) + (y3 << 18)` (wrapping), where y1, y2, y3 are three
    /// consecutive generator values; the state carries from entry to entry, starting at 0.
    pub fn build() -> HashTable {
        let mut t = [0u32; HASH_TABLE_LEN];
        let mut state: u32 = 0;
        for slot in t.iter_mut() {
            let y1 = lcg_step(state);
            let y2 = lcg_step(y1);
            let y3 = lcg_step(y2);
            *slot = (y1 >> 2).wrapping_add(y2 << 8).wrapping_add(y3 << 18);
            state = y3;
        }
        HashTable { t, tail: state }
    }

    /// The table as 1,024 little-endian bytes.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.t.iter().flat_map(|v| v.to_le_bytes()).collect()
    }

    pub fn hash1(&self, x: u32) -> u32 {
        hash1(&self.t, x)
    }
    pub fn hash2(&self, x: u32, y: u32) -> u32 {
        hash2(&self.t, x, y)
    }
}

#[inline(always)]
fn round(t: &[u32; HASH_TABLE_LEN], v: u32) -> u32 {
    (v >> 8).wrapping_add(t[(v & 0xff) as usize])
}

/// 0x5b9930: four rounds of `v = (v >> 8) + T[v & 0xff]` (wrapping u32).
pub fn hash1(t: &[u32; HASH_TABLE_LEN], x: u32) -> u32 {
    let mut v = x;
    for _ in 0..4 {
        v = round(t, v);
    }
    v
}

/// 0x5b9970: four rounds on x, `v += y` (wrapping), four more rounds.
pub fn hash2(t: &[u32; HASH_TABLE_LEN], x: u32, y: u32) -> u32 {
    let mut v = x;
    for _ in 0..4 {
        v = round(t, v);
    }
    v = v.wrapping_add(y);
    for _ in 0..4 {
        v = round(t, v);
    }
    v
}
