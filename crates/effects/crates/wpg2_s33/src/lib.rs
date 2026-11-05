//! WPG2 (2026-11-02): class warp slot 33 as a named Rust function.
//!
//! Route (MEASURED from the listing, see README): slot 33 = (context build, optional image geometry override with,
//! flag byte rule, host plane service call `[[ctx]]`then with r9d = 0), (two planes: v = (hi - lo) * 0.25f,
//! eight cell stores), parallel_for (callback), the callback's tile partition (ported here as REAL code,
//! `partition`) and the AVX2 body (`prefill::run_body`WS4 copy without mutation switches).
//!
//! Engine statement: the emulator follows the Intel SDM, not silicon. ROUTE AVX2-ONLY (the callback's feature word is an explicit tier input).
//! SERIAL-WORKERS: the pool runner is modelled by running each worker's tile one after the other (tile independence is covered by
//! the body T2: partial tiles). HOST-FP-ASSUMED. Floats are carried as u32 bit patterns.

pub mod prefill;
use prefill::{mul_ss, sub_ss, BodyError, Params};
pub const SENT: u32 = prefill::SENTINEL;

/// 0.25f, CAPTURED.
pub const QUARTER: u32 = 0x3e80_0000;
/// Callback dispatcher masks : AVX2 body when `w & 0x9d97ff == 0x9d97ff`.
pub const FW_AVX2: u64 = 0x9d97ff;
pub const FW_MID: u64 = 0x117ff;
/// Log strings (listing addresses; edi = 9 in every log call).
pub mod judge_keys;
pub use judge_keys::{STR_ASPECT0, STR_G_CELL_A_ZERO, STR_NESTED}; // pool state g_cell_b == 0
 // g_cell_d != 0
 // aspect == 0 in 

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tier {
    Avx2,
    AvxMiddle,
    Sse,
    CpuInit,
}

pub fn tier(w: u64) -> Tier {
    if w & FW_AVX2 == FW_AVX2 {
        Tier::Avx2
    } else if w & FW_MID == FW_MID {
        Tier::AvxMiddle
    } else if w & 1 != 0 {
        Tier::Sse
    } else {
        Tier::CpuInit
    }
}

/// Events seen by the service hooks, in order (the call record of the T2).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Ev {
    /// host plane service `[[ctx]]` (edx = 2, ecx = 2, r8d = 0, two zero stack words)
    Host,
    /// (edi, rsi): log call
    Log(u32, u32),
    /// thread count
    Nthr,
    /// thread id
    Tid,
    /// one-time pool init calls (in this order)
    Init1,
    Init2,
    Init3,
    /// pool runner
    Runner,
}

pub fn tok(e: &Ev) -> String {
    match e {
        Ev::Host => "H".into(),
        Ev::Log(a, b) => format!("G{:x}:{:x}", a, b),
        Ev::Nthr => "C".into(),
        Ev::Tid => "T".into(),
        Ev::Init1 => "I1".into(),
        Ev::Init2 => "I2".into(),
        Ev::Init3 => "I3".into(),
        Ev::Runner => "R".into(),
    }
}

/// Pool state: bss cells (listing addresses) g_cell_b =, g_cell_c =, g_cell_d =, g_cell_e =, init result, and
/// the two host values . All are PARAM-UNCAPTURED run-time state: the captured image holds
/// zeros for the bss ones.
#[derive(Clone, Debug, Default)]
pub struct Pool {
    pub g_cell_b: i32,
    pub g_cell_c: i32,
    pub g_cell_d: i32,
    pub g_cell_e: i32,
    pub g_cell_a: u64,
    pub wpg2_s33_pool_b: i32,
    pub wpg2_s33_pool_a: i32,
    /// return value of (stored into g_cell_a by the init)
    pub init_ret: u64,
}

/// Return values of the thread helpers: (first and second call of a partition) and .
#[derive(Clone, Copy, Debug)]
pub struct Threads {
    pub nthr: i32,
    pub nthr2: i32,
    pub tid: i32,
}

#[derive(Debug, PartialEq)]
pub enum S33Error {
    /// idiv by zero in (thread count 0)
    DivZero,
    Body(BodyError),
    /// the dispatcher picked a tier whose code is not ported (ROUTE AVX2-ONLY)
    RouteNotRebuilt(Tier),
    /// a plane is shorter than width * height
    Short,
}

/// (edi = total, rsi = &start, rdx = &count), ported in full. Branch notes: the init block is unreachable from 
/// (g_cell_b == 0 there implies g_cell_c != 0 because the first block ran the init), and the tests `g_cell_d == 0` and `wpg2_s33_pool_a == 0` are
/// implied by earlier tests; they are kept as code.
pub fn partition(total: i32, pool: &mut Pool, th: &Threads, ev: &mut Vec<Ev>) -> Result<(i32, i32), S33Error> {
    if pool.g_cell_d == 0 {
        return Ok((0, total)); // start = 0, count = total
    }
    if pool.g_cell_b == 0 && pool.g_cell_c == 0 {
        init(pool, ev); // 
    }
    // 
    let r15;
    if pool.wpg2_s33_pool_b != 0 && pool.wpg2_s33_pool_a != 0 {
        ev.push(Ev::Nthr);
        r15 = th.nthr;
    } else {
        r15 = 1;
    }
    let ecx: i32;
    if pool.wpg2_s33_pool_b == 0 || pool.wpg2_s33_pool_a == 0 || pool.g_cell_d == 0 {
        ecx = 0; // 
    } else if pool.g_cell_b == 0 && pool.g_cell_c == 0 {
        init(pool, ev); //  (unreachable, kept)
        ecx = tid_of(pool, th, ev);
    } else {
        ecx = tid_of(pool, th, ev); // 
    }
    if total < r15 {
        let edx: i32 = if ecx < total { 1 } else { 0 }; // 
        let start = ecx.wrapping_mul(edx);
        Ok((start, edx)) // [r13] = start, [r14] = count
    } else {
        let q = total.checked_div(r15).ok_or(S33Error::DivZero)?; //  idiv (a zero divisor faults in the original)
        let last = r15.wrapping_sub(1);
        let start = q.wrapping_mul(ecx);
        let mut count = q;
        if ecx == last {
            count = total.wrapping_sub(start);
        }
        Ok((start, count))
    }
}

fn tid_of(pool: &Pool, th: &Threads, ev: &mut Vec<Ev>) -> i32 {
    if pool.wpg2_s33_pool_a == 0 {
        return 0;
    }
    ev.push(Ev::Nthr);
    if th.nthr2 > 1 {
        ev.push(Ev::Tid);
        th.tid
    } else {
        0
    }
}

fn init(pool: &mut Pool, ev: &mut Vec<Ev>) {
    ev.push(Ev::Init1);
    pool.g_cell_a = pool.init_ret;
    ev.push(Ev::Init2);
    pool.g_cell_c = 1;
    ev.push(Ev::Init3);
    pool.g_cell_b = 1;
}

/// (this, callback): the callback runs with the pool state `poolcb(pool, threads, ev)` is one worker's call of .
pub fn parallel_for<F>(pool: &mut Pool, th: &Threads, ev: &mut Vec<Ev>, mut cb: F) -> Result<(), S33Error>
where
    F: FnMut(&mut Pool, &Threads, &mut Vec<Ev>) -> Result<(), S33Error>,
{
    if pool.g_cell_b == 0 {
        ev.push(Ev::Log(9, STR_G_CELL_A_ZERO));
        return cb(pool, th, ev);
    }
    if pool.g_cell_d != 0 {
        ev.push(Ev::Log(9, STR_NESTED));
        return cb(pool, th, ev);
    }
    pool.g_cell_d = 1;
    let res = if pool.wpg2_s33_pool_b != 0 && pool.wpg2_s33_pool_a != 0 {
        ev.push(Ev::Nthr);
        if th.nthr > 1 {
            pool.g_cell_e = 1;
            ev.push(Ev::Runner);
            let mut r = Ok(());
            for tid in 0..th.nthr {
                let t = Threads { tid, ..*th };
                r = cb(pool, &t, ev);
                if r.is_err() {
                    break;
                }
            }
            pool.g_cell_e = 0;
            r
        } else {
            cb(pool, th, ev)
        }
    } else {
        cb(pool, th, ev)
    };
    pool.g_cell_d = 0;
    res
}

/// One callback call (over body) on plane `input` into `output`: partition then run the body on the tile.
pub fn callback(fw: u64, p: &Params, input: &[u32], output: &mut [u32], pool: &mut Pool, th: &Threads, ev: &mut Vec<Ev>) -> Result<(), S33Error> {
    match tier(fw) {
        Tier::Avx2 => {}
        other => return Err(S33Error::RouteNotRebuilt(other)),
    }
    let (start, count) = partition(p.height, pool, th, ev)?;
    let q = Params { start, count, ..p.clone() };
    prefill::run_body(&q, input, output).map_err(S33Error::Body)
}

/// The fields of the byte record that slot 33 reads (copied by into the context): gate, width, height,
/// (f32 bits).
#[derive(Clone, Copy, Debug)]
pub struct Rec33 {
    pub gate: i32,
    pub w: i32,
    pub h: i32,
    pub assumption_f: u32,
    pub assumption_g: u32,
    pub assumption_h: u32,
    pub geom33_a: u32,
}

/// The image object's geometry block, read when byte [img] != 0 : [img] -> ctx, [img],
/// [img], [img]; [img] is the aspect argument of .
#[derive(Clone, Copy, Debug)]
pub struct Geom33 {
    pub assumption_f: u32,
    pub assumption_g: u32,
    pub assumption_h: u32,
    pub geom33_a: u32,
    pub aspect: u32,
}

/// The primitive instance fields slot 33 reads: i32 [P] (plane count / filter), byte [P] (constant coordinates).
#[derive(Clone, Copy, Debug)]
pub struct Prim33 {
    pub n: i32,
    pub cc: u8,
}

#[derive(Debug, PartialEq)]
pub struct Out33 {
    /// byte stored to [P]
    pub flag_byte: u8,
    /// true when the host service ran and was called
    pub ran: bool,
}

/// Slot 33 of class warp . `rec` is None for a null record pointer (zero-fills the context). `uv` = the two coordinate
/// planes [[P]] and; `c1c2` = the two planes the host service returns (written through the prefill body).
#[allow(clippy::too_many_arguments)]
pub fn slot33_warp(
    fw: u64,
    prim: &Prim33,
    rec: Option<&Rec33>,
    geom: Option<&Geom33>,
    uv: (&[u32], &[u32]),
    c1: &mut [u32],
    c2: &mut [u32],
    pool: &mut Pool,
    th: &Threads,
    ev: &mut Vec<Ev>,
) -> Result<Out33, S33Error> {
    // copy of the record, or zero fill for a null pointer
    let mut ctx = rec.copied().unwrap_or(Rec33 { gate: 0, w: 0, h: 0, assumption_f: 0, assumption_g: 0, assumption_h: 0, geom33_a: 0 });
    if let Some(g) = geom {
        ctx.assumption_f = g.assumption_f;
        ctx.assumption_g = g.assumption_g;
        ctx.assumption_h = g.assumption_h;
        ctx.geom33_a = g.geom33_a;
        if f32::from_bits(g.aspect) == 0.0 {
            ev.push(Ev::Log(9, STR_ASPECT0)); //  error path (aspect +0 or -0)
        }
    }
    let flag = prim.n != 0 && prim.cc == 0;
    let flag_byte = flag as u8;
    if ctx.gate != 0 || !flag {
        return Ok(Out33 { flag_byte, ran: false });
    }
    ev.push(Ev::Host);
    let r8 = if prim.n >= 3 { 2 } else if prim.n >= 2 { 1 } else { 0 };
    // plane 1: in = u, out = c1, v1 = (assumption_g - assumption_f) * 0.25
    let v1 = mul_ss(sub_ss(ctx.assumption_g, ctx.assumption_f), QUARTER);
    let mk = |v: u32, fl: u32| Params { width: ctx.w, height: ctx.h, flag: fl, pre_cell_a_bits: 0, pre_cell_b_bits: 0, v, start: 0, count: 0 };
    let need = (ctx.w.max(0) as usize) * (ctx.h.max(0) as usize);
    if uv.0.len() < need || uv.1.len() < need || c1.len() < need || c2.len() < need {
        return Err(S33Error::Short);
    }
    let p1 = mk(v1, (r8 > 0) as u32);
    parallel_for(pool, th, ev, |pl, t, e| callback(fw, &p1, uv.0, c1, pl, t, e))?;
    // plane 2: in = v, out = c2, v2 = (geom33_a - assumption_h) * 0.25
    let v2 = mul_ss(sub_ss(ctx.geom33_a, ctx.assumption_h), QUARTER);
    let p2 = mk(v2, (r8 > 1) as u32);
    parallel_for(pool, th, ev, |pl, t, e| callback(fw, &p2, uv.1, c2, pl, t, e))?;
    Ok(Out33 { flag_byte, ran: true })
}

pub fn assumptions() -> Vec<(&'static str, &'static str)> {
    vec![
        ("WPG2-A1", "the host plane service `[[ctx]]` (edx = 2, ecx = 2) returns two planes c1, c2 of width * height floats (stride = width); it is a host service hook (W69), not plugin code; the planes are inputs of the port"),
        ("WPG2-A2", "pool state (g_cell_b, g_cell_c, g_cell_d, g_cell_e) and the thread helper returns are PARAM-UNCAPTURED run-time state; the captured image holds zeros (g_cell_b = 0: serial callback with a log). T2 varies each"),
        ("WPG2-A3", "SERIAL-WORKERS: the pool runner is modelled as the workers' tiles run one after the other, tids 0..nthr-1 (INFERRED that a real pool uses ids 0..n-1)"),
        ("WPG2-A4", "ROUTE AVX2-ONLY: the callback dispatcher feature word is an explicit input; AVX middle tier does not exist for this callback, the SSE twin is not ported"),
        ("WPG2-A5", "memcpy and memset inside are functional models in the emulator T2 (LIBC-MODEL); sets aspect_block_d etc. which slot 33 does not read; is the round_q12 model"),
        ("WPG2-A6", "HOST-FP-ASSUMED: round to nearest, no FTZ, no DAZ; NaN rules follow SSE silicon, FMA NaN priority INFERRED (WS4-A7)"),
    ]
}
