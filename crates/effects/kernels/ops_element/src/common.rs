//! Shared pieces of the classes: x86 max/min rules, the host plane service call (, read-only dependency), errors, mutants, assumptions.
use plane_service::{plane_service, Block, Host, PlError, Roi, Sel};

/// Errors: the original asserts for an operand count outside 1..4; the port returns an error instead.
#[derive(Clone, Debug, PartialEq)]
pub enum ElError {
    BadCount(i32),
    BadNout(i32),
    PlaneMissing,
    PlaneShort,
    DstShort,
    Service(PlError),
}

impl From<PlError> for ElError {
    fn from(e: PlError) -> Self {
        ElError::Service(e)
    }
}

/// What the render did (compared with the emulated run: number of entries into the plane service).
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Trace {
    pub service_calls: u32,
}

/// Host state of the plane service callee for descriptor 0 (the argument block returned by [vtable] = base getter, MEASURED:
/// `[this]*mode`). All fields EXPLICIT with no default (EM1-A4, EM1-A5). `is_intvaluesenable` matter only when the operand
/// is a constant and the body hands it to the service (max_element/min_element/element_of with count 1: the tail jump to).
#[derive(Clone, Copy, Debug)]
pub struct Svc {
    pub in_pool: bool,
    pub is_int: bool,
    pub values: [u32; 4],
    pub enable: bool,
    pub img_valid: bool,
}

/// MAXSS/VMAXPS rule (SDM): result = src1 > src2 ? src1 : src2, so a NaN in either operand and a pair of zeros return src2.
#[inline]
pub fn x86_max(a: f32, b: f32) -> f32 {
    if a > b {
        a
    } else {
        b
    }
}

/// MINSS/VMINPS rule (SDM): result = src1 < src2 ? src1 : src2.
#[inline]
pub fn x86_min(a: f32, b: f32) -> f32 {
    if a < b {
        a
    } else {
        b
    }
}

/// CVTTSS2SI r32: truncation toward zero; NaN and out of range give the integer indefinite 0x80000000.
#[inline]
pub fn cvtt(x: f32) -> i32 {
    if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
        i32::MIN
    } else {
        x as i32
    }
}

/// Signed remainder by 3 exactly as the listing computes it : q = hi32(i * 0x55555556) - (i >> 31), r = i - 3q.
#[inline]
pub fn rem3_raw(i: i32) -> i32 {
    let p = (i as i64) * 0x5555_5556i64;
    let hi = (p >> 32) as i32;
    let q = hi.wrapping_sub(i >> 31);
    i.wrapping_sub(q.wrapping_mul(3))
}

/// Calls the plane service for descriptor 0: mode 0, `chan`, flat ROI of `n` elements. `planes` are the operand's image planes.
pub fn call_service(svc: &Svc, count: i32, planes: &[&[f32]], chan: i32, dst: &mut [f32], n: i32, tr: &mut Trace) -> Result<(), ElError> {
    tr.service_calls += 1;
    let block = Block { is_int: svc.is_int, values: &svc.values, enable: svc.enable, maxch: count, img_valid: svc.img_valid, planes };
    let roi = Roi { mode: 0, n, port_blk_a: 0, port_blk_b: 0, port_blk_c: 0 };
    plane_service(&Host { in_pool: svc.in_pool }, Sel::Virtual { block }, 0, chan, dst, &roi)?;
    Ok(())
}

pub fn assumptions() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        ("EM1-A1", "CAPTURED/none", "the three AVX2 bodies read no data cells (MEASURED scan: only the feature word cell and assert strings); no ONE, no vector constant"),
        ("EM1-A2", "ROUTE", "AVX2 route only (feature word 0x9d97ff): bodies,; AVX and SSE tiers not compared"),
        ("EM1-A3", "HOST-FP-ASSUMED", "MXCSR round to nearest; the bodies only compare and copy, no rounding"),
        ("EM1-A4", "CALLEE-MODELLED/PL1", "plane service is the finished native; its descriptor-0 block (class virtual returns [this]) is an explicit input (Svc), host fill INFERRED"),
        ("EM1-A5", "PARAM-UNCAPTURED", "Svc.in_pool (pool flag cell): both values run; matters only for a constant fill of >= 512 floats (-0.0 becomes +0.0)"),
        ("EM1-A6", "HOST-HOOKED", "virtual slot 31 (call [rax]) of element_of is a count getter: nout is an explicit parameter; max_element/min_element never call it"),
        ("EM1-A7", "INFERRED host fill", "the operand record at this (count, flag, consts, plane record) and descriptor 0 at [this] describe the same operand"),
        ("EM1-A8", "contract", "operand count outside 1..4 (original: assert) and nout outside 1..4 for element_of return an error; n <= 0 writes nothing for count >= 2"),
    ]
}
