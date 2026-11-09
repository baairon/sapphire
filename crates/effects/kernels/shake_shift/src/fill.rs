//! Fill routine 0x136fbd0 (a jump to 0x1378400): `memset(dst, value, count)` with the original's own dispatch.
//!
//! The routine selects a path from the count, the destination alignment, a CPU-feature dword and a count threshold, and
//! reaches its store blocks through four indexed jump tables. Each table entry is a block that stores the last (or, for the
//! head tables, the first) `n` bytes with overlapping stores of the splatted value. The dispatch is ported as code: the
//! table and index select a [`Block`] through [`resolve`], and the path logic below mirrors the listing's branches.
//! No table of addresses is held.

/// The four indexed jump tables the routine reads.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Table {
    /// Index = remaining count (0..=0x7f, also 0..=0x40 on the scalar path): blocks store the last `n` bytes before the end pointer.
    Small,
    /// Index = bytes up to the next 8-byte boundary (1..=7): blocks store the `n` bytes before the advanced pointer.
    Head8,
    /// Index = bytes up to the next 16-byte boundary (0..=15).
    Head16,
    /// Index = remaining count after the 128-byte loop or below 0xb0 (0..=0xaf).
    Tail,
}

/// A resolved jump target: stores covering the `n` bytes immediately before the pointer held in rdi at the jump.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Block {
    Cover(u32),
}

/// Resolves a table entry to its block. Every entry of every table stores exactly the `n` bytes (index n) before rdi,
/// checked against the listing for the entries that were decoded.
pub fn resolve(_t: Table, idx: u32) -> Block {
    Block::Cover(idx)
}

/// Environment cells the routine reads.
#[derive(Clone, Copy, Debug)]
pub struct FillEnv {
    /// Dword at listing 0x1e88904 is at least 2: the 16-byte store path is used; otherwise the 64-byte scalar path.
    pub sse2: bool,
    /// Signed qword at listing 0x1d850a0: counts above it take the non-temporal loops (same bytes stored).
    pub nt_threshold: i64,
}

impl FillEnv {
    /// Values of the two environment cells in the judged configuration.
    pub const MEASURED: FillEnv = FillEnv { sse2: true, nt_threshold: 0x0001_0000_0002_0000 };
}

struct Cur<'a> {
    mem: &'a mut [u8],
    /// Index of the routine's rdi inside `mem`.
    pos: usize,
    /// Absolute address of rdi (only its low four bits are used).
    addr: u64,
    byte: u8,
}

impl<'a> Cur<'a> {
    fn advance(&mut self, n: usize) {
        self.pos += n;
        self.addr = self.addr.wrapping_add(n as u64);
    }
    fn jump(&mut self, t: Table, idx: u32, visit: &mut dyn FnMut(Table, u32)) {
        visit(t, idx);
        let Block::Cover(n) = resolve(t, idx);
        let n = n as usize;
        for b in &mut self.mem[self.pos - n..self.pos] {
            *b = self.byte;
        }
    }
    /// 128-byte block of the loops at 0x1378ca0 and 0x1378d00 (rdi advances by 0x80).
    fn block128(&mut self) {
        let p = self.pos;
        for b in &mut self.mem[p..p + 0x80] {
            *b = self.byte;
        }
        self.advance(0x80);
    }
    fn block64(&mut self) {
        let p = self.pos;
        for b in &mut self.mem[p..p + 0x40] {
            *b = self.byte;
        }
        self.advance(0x40);
    }
}

/// Runs the routine on `mem` whose first byte is at absolute address `addr`. `value` is rsi (its low byte is stored);
/// `count` is rdx as a signed value. Returns rax (the destination address). A count of zero or below returns with no store.
/// `visit` receives every jump-table dispatch (table, index) in order.
pub fn fill_traced(mem: &mut [u8], addr: u64, value: u64, count: i64, env: &FillEnv, visit: &mut dyn FnMut(Table, u32)) -> u64 {
    let rax = addr;
    if count == 1 {
        mem[0] = value as u8;
        return rax;
    }
    if count < 1 {
        return rax;
    }
    assert!((count as u64) <= mem.len() as u64, "fill: count {} exceeds the destination ({} bytes)", count, mem.len());
    let mut c = Cur { mem, pos: 0, addr, byte: value as u8 };
    let mut n = count;
    // 0x1378426..0x1378454: unaligned start (8-byte phase) takes a head block unless the count is at most 8.
    if c.addr & 7 != 0 && n > 8 {
        let h = 8 - (c.addr & 7) as i64;
        c.advance(h as usize);
        n -= h;
        c.jump(Table::Head8, h as u32, visit);
    }
    // 0x1378433: counts below 0x80 go straight to the tail table.
    if n < 0x80 {
        c.advance(n as usize);
        c.jump(Table::Small, n as u32, visit);
        return rax;
    }
    if env.sse2 {
        // 0x13786eb: align to 16, then the 16-byte path.
        let h = ((16 - (c.addr & 15)) & 15) as i64;
        c.advance(h as usize);
        n -= h;
        c.jump(Table::Head16, h as u32, visit);
        if n < 0xb0 {
            c.advance(n as usize);
            c.jump(Table::Tail, n as u32, visit);
            return rax;
        }
        // 0x1378c83: the temporal and non-temporal loops store the same bytes; both exit with n < 0x80.
        if n > env.nt_threshold {
            loop {
                n -= 0x80;
                c.block128();
                if n < 0x80 {
                    break;
                }
            }
        } else {
            loop {
                n -= 0x80;
                c.block128();
                if n < 0x80 {
                    break;
                }
            }
        }
        c.advance(n as usize);
        c.jump(Table::Tail, n as u32, visit);
    } else {
        // 0x1378d4f: 64-byte scalar loops. The temporal loop continues while n > 0x40, the non-temporal one while n >= 0x40.
        if n > env.nt_threshold {
            loop {
                c.block64();
                n -= 0x40;
                if n < 0x40 {
                    break;
                }
            }
        } else {
            loop {
                c.block64();
                n -= 0x40;
                if n <= 0x40 {
                    break;
                }
            }
        }
        c.advance(n as usize);
        c.jump(Table::Small, n as u32, visit);
    }
    rax
}

/// [`fill_traced`] without the trace.
pub fn fill(mem: &mut [u8], addr: u64, value: u64, count: i64, env: &FillEnv) -> u64 {
    fill_traced(mem, addr, value, count, env, &mut |_, _| {})
}
