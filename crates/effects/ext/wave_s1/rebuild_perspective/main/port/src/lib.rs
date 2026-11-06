//! Native port of the AVX2 body (listing address) of the class `perspective`slot 52.
//! Listing addresses are used for code; a captured data VA is the listing offset.
//!
//! What it computes (MEASURED from the listing): for each element i of two input arrays
//! (u = [[this]], v = [[this]]) it writes two output arrays
//! out1[i] = (b*u[i]) * q and out2[i] = (v[i]*m5 - m6) * q + m1, where
//! (m6, m7) = (r0, r1) of the callee (INFERRED sincosf) at arg = (-0.25 * gpu_call_a) * 2pi,
//! m1 = a*m6, P = a*m7, m4 = m6*b, m5 = m7*b, d = (-m4)*v - m7 (fused), and q is a
//! sign-dependent reciprocal of d (see `q_scalar`, `q_vector`). a = port_obj_c, b = port_obj_d.
//! The AVX2 body has scalar, 8 wide and 16 wide parts selected by n and the low 5 bits of
//! the out2 pointer (`out2_phase_bytes`). The flag path (cl != 0) calls the GPU stub .

/// Assumption record (stand-ins, never data requests).
pub struct Assumption {
    pub id: &'static str,
    pub text: &'static str,
}

pub fn assumptions() -> Vec<Assumption> {
    vec![
        Assumption { id: "PX1-A1", text: "RETIRED (RERUN_SC1): the callee is the native crate libm_sincos, bit exact against the emulated routine; the f64 sin/cos model differed from it in 4 of 18,540 finite inputs (0 on the perspective range). Inherited labels: SC1-A1 (sincosf table entries 16..63) and SC1-A2 (double table entries 4..63), both ASSUMED-SEMANTIC, carried by the emulator image on the original side and by libm_sincos on the Rust side." },
        Assumption { id: "PX1-A2", text: "APPROX-MODEL: vrcpps (6 sites:) is modelled as the exact f32 reciprocal 1.0/x on both sides; the code's own Newton step follows. The SDM bound of the estimate is 1.5 * 2^-12 relative; the Rust-only sensitivity test tests/approx.rs measures the output change (APPROX-BOUND, t2/APPROX_BOUND_SC1.txt)." },
        Assumption { id: "PX1-A3", text: " (uncovered) are 32 bit integer 1 in every lane (INFERRED: they are ANDed with a cmpgt mask and then compared by vpcmpeqd with a 0/1 broadcast of the scalar sign test). Same stand-in on both sides." },
        Assumption { id: "PX1-A4", text: " (uncovered) is all ones in every lane (same label as SC2-A1 of the dissolve port); used as a NOT mask (vxorps) so only its use as all ones matters." },
        Assumption { id: "PX1-A5", text: " holds 2pi as f32 0x40c90fdb in its first 4 bytes (MEASURED, lookup_raw.tsv: the other 12 bytes are uncovered and are not read by vmulss)." },
        Assumption { id: "PX1-A6", text: "the cl flag path (cl != 0) is a call to (GPU launch stub: ROI, image allocs) and is not ported; run returns Err(Path::GpuStub) with the argument record. A null [this] faults in the original (reads [0]) and is returned as Err(Path::NullInputs)." },
    ]
}

/// IEEE constants read from the data image (MEASURED, lookup_raw.tsv).
pub const C_QUARTER_NEG: f32 = -0.25; // bytes 000080be
pub const C_TWO_PI: f32 = f32::from_bits(0x40c9_0fdb); // 
pub const C_ONE_HALF_PLUS: f32 = f32::from_bits(0x3fc0_0000); //  = 1.5f (bytes 0000c03f)

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Obj {
    pub port_obj_c: f32,
    pub gpu_call_a: f32,
    pub port_obj_d: f32,
}

#[derive(Debug, PartialEq)]
pub enum Path {
    NullInputs,
    GpuStub(GpuCall),
}

/// Arguments the body hands to (rdi = this, rsi = args record, rdx = outs, xmm0 = 0, xmm3 = b).
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct GpuCall {
    pub b: f32,
}

/// Argument of the callee (xmm0).
pub fn sincos_arg(gpu_call_a: f32) -> f32 {
    (C_QUARTER_NEG * gpu_call_a) * C_TWO_PI
}

/// The one native for the callee (crate libm_sincos): (r0, r1) = (sin, cos) as f32, bit exact against the emulated routine.
/// PX1-A1 (the f64 model) is retired: the callee is the native crate libm_sincos.
pub fn sincos_native(x: f32) -> (f32, f32) {
    libm_sincos::sincosf_x(x)
}

/// Per-call constants after the callee returns.
#[derive(Clone, Copy, Debug)]
pub struct K {
    pub m1: f32,
    pub p: f32,
    pub m4: f32,
    pub m5: f32,
    pub m6: f32,
    pub m7: f32,
    pub b: f32,
}

pub fn constants(o: &Obj, r0: f32, r1: f32) -> K {
    K { m1: o.port_obj_c * r0, p: o.port_obj_c * r1, m4: r0 * o.port_obj_d, m5: r1 * o.port_obj_d, m6: r0, m7: r1, b: o.port_obj_d }
}

fn d_of(k: &K, v: f32) -> f32 {
    (-k.m4).mul_add(v, -k.m7) // vfnmsub213ss / vfmsub213ps with the negated m4
}
fn dneg_of(k: &K, v: f32) -> f32 {
    k.m4.mul_add(v, k.m7) // vfnmadd213ps (-(-m4 * v) + m7)
}

/// Scalar parts (prologue peel, tails).
pub fn q_scalar(k: &K, v: f32) -> f32 {
    let d = d_of(k, v);
    if d == 0.0 {
        return k.p;
    }
    let dgt = d > 0.0;
    if (k.p > 0.0) == dgt {
        (C_ONE_HALF_PLUS - k.p) / d
    } else {
        k.p / (-d)
    }
}

/// The six vrcpps sites of the body (listing addresses) and where each sits: the 16 wide loop handles two ymm blocks (elements 0..7 and 8..15
/// of each 16 block, "A" and "B"), each with a d branch and a -d branch; the 8 wide tail has one of each.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RcpSite {
    /// 16 wide, block A, d
    S_CELL_A,
    /// 16 wide, block B, d
    SCellA,
    /// 16 wide, block A, -d
    S_CELL_B,
    /// 16 wide, block B, -d
    S_CELL_C,
    /// 8 wide tail, d
    SCellB,
    /// 8 wide tail, -d
    SCellC,
}
pub const RCP_SITES: [RcpSite; 6] = [RcpSite::S_CELL_A, RcpSite::SCellA, RcpSite::S_CELL_B, RcpSite::S_CELL_C, RcpSite::SCellB, RcpSite::SCellC];

/// Where an element sits in the section plan.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Part {
    Scalar,
    Wide16A,
    Wide16B,
    Wide8,
}

fn rcp_exact(_s: RcpSite, x: f32) -> f32 {
    1.0f32 / x // PX1-A2 (APPROX-MODEL): the exact f32 reciprocal stands for vrcpps on both sides
}
fn nr_inv(s: RcpSite, x: f32, rcp: &dyn Fn(RcpSite, f32) -> f32) -> f32 {
    let r = rcp(s, x);
    (r + r) - ((r * x) * r) // the code's own Newton step: 2r - (r*x)*r
}

/// 8 and 16 wide parts (reciprocal by vrcpps and one Newton step, no division). `part` selects the vrcpps site.
pub fn q_vector_with(k: &K, v: f32, part: Part, rcp: &dyn Fn(RcpSite, f32) -> f32) -> f32 {
    let (sd, sn) = match part {
        Part::Wide16B => (RcpSite::SCellA, RcpSite::S_CELL_C),
        Part::Wide8 => (RcpSite::SCellB, RcpSite::SCellC),
        _ => (RcpSite::S_CELL_A, RcpSite::S_CELL_B),
    };
    let d = d_of(k, v);
    let zero = d == 0.0;
    let dgt = d > 0.0;
    let pgt = k.p > 0.0;
    if zero {
        k.p
    } else if pgt == dgt {
        (C_ONE_HALF_PLUS - k.p) * nr_inv(sd, d, rcp)
    } else {
        k.p * nr_inv(sn, dneg_of(k, v), rcp)
    }
}

pub fn q_vector(k: &K, v: f32) -> f32 {
    q_vector_with(k, v, Part::Wide16A, &rcp_exact)
}

fn elem(k: &K, u: f32, v: f32, part: Part, rcp: &dyn Fn(RcpSite, f32) -> f32) -> (f32, f32) {
    let q = if part == Part::Scalar { q_scalar(k, v) } else { q_vector_with(k, v, part, rcp) };
    let t = v.mul_add(k.m5, -k.m6); // vfmsub213ss / ps
    ((k.b * u) * q, t.mul_add(q, k.m1))
}

/// Section plan: (peel scalar count, 16 wide end, 8 wide end, scalar start of the tail).
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Plan {
    pub peel: usize,
    pub v16_end: usize,
    pub v8_end: usize,
    pub n: usize,
}

pub fn plan(n: i32, out2_phase_bytes: u32) -> Plan {
    if n <= 0 {
        return Plan { peel: 0, v16_end: 0, v8_end: 0, n: 0 };
    }
    let nn = n as usize;
    let all_scalar = Plan { peel: nn, v16_end: nn, v8_end: nn, n: nn }; // peel covers all
    if n < 16 {
        return all_scalar;
    }
    let (peel, esi);
    if n < 29 {
        peel = 0usize;
        esi = nn & !0xf;
    } else {
        let ph = out2_phase_bytes & 0x1f;
        let p = if ph == 0 {
            0
        } else if ph & 3 != 0 {
            return all_scalar;
        } else {
            ((32 - ph) / 4) as usize
        };
        if nn < p + 16 {
            return all_scalar;
        }
        peel = p;
        esi = nn - ((nn - p) & 0xf);
    }
    let rem = nn - esi;
    let v8_end = if rem >= 8 { esi + (rem & !7) } else { esi };
    Plan { peel, v16_end: esi, v8_end, n: nn }
}

/// Which section an element i belongs to under a plan (the 16 block parity counts from the end of the peel).
pub fn part_of(pl: &Plan, i: usize) -> Part {
    if pl.peel < pl.n && i >= pl.peel && i < pl.v16_end {
        if (i - pl.peel) % 16 < 8 { Part::Wide16A } else { Part::Wide16B }
    } else if i >= pl.v16_end && i < pl.v8_end {
        Part::Wide8
    } else {
        Part::Scalar
    }
}

/// Run the body with the vrcpps estimate injectable (APPROX-BOUND test). `run` uses the exact reciprocal. Same arguments as `run`.
#[allow(clippy::too_many_arguments)]
pub fn run_with_rcp(
    o: &Obj,
    has_inputs: bool,
    flag: bool,
    n: i32,
    r: (f32, f32),
    out2_phase_bytes: u32,
    u: &[f32],
    v: &[f32],
    out1: &mut [f32],
    out2: &mut [f32],
    rcp: &dyn Fn(RcpSite, f32) -> f32,
) -> Result<(), Path> {
    // the callee is called before any branch, the flag test comes after
    if flag {
        return Err(Path::GpuStub(GpuCall { b: o.port_obj_d }));
    }
    if n <= 0 {
        return Ok(());
    }
    if !has_inputs {
        return Err(Path::NullInputs);
    }
    let k = constants(o, r.0, r.1);
    let pl = plan(n, out2_phase_bytes);
    for i in 0..pl.n {
        let (a, b) = elem(&k, u[i], v[i], part_of(&pl, i), rcp);
        out1[i] = a;
        out2[i] = b;
    }
    Ok(())
}

/// Run the body with an explicit callee result `r` (tests with fixed values). `uv` are [[this]] / .
/// `has_inputs` is false when [this] is null. `flag` is cl.
#[allow(clippy::too_many_arguments)]
pub fn run(
    o: &Obj,
    has_inputs: bool,
    flag: bool,
    n: i32,
    r: (f32, f32),
    out2_phase_bytes: u32,
    u: &[f32],
    v: &[f32],
    out1: &mut [f32],
    out2: &mut [f32],
) -> Result<(), Path> {
    run_with_rcp(o, has_inputs, flag, n, r, out2_phase_bytes, u, v, out1, out2, &rcp_exact)
}

/// The body as shipped: the callee is the native libm_sincos at arg = (-0.25 * gpu_call_a) * 2pi (the flag path returns before any use of it).
#[allow(clippy::too_many_arguments)]
pub fn run_full(
    o: &Obj,
    has_inputs: bool,
    flag: bool,
    n: i32,
    out2_phase_bytes: u32,
    u: &[f32],
    v: &[f32],
    out1: &mut [f32],
    out2: &mut [f32],
) -> Result<(), Path> {
    let r = sincos_native(sincos_arg(o.gpu_call_a));
    run(o, has_inputs, flag, n, r, out2_phase_bytes, u, v, out1, out2)
}
