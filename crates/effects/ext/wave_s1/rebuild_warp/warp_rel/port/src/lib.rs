//! Class warp_rel (vtable), CPU render entry: slot 53 = wrapper (160 B) over F2 (AVX2 body).
//! Source of truth: neutral_1.asm (listing), see SPEC.md. Engine statement: the emulator follows the SDM, not silicon.
//! ROUTE AVX2-ONLY, HOST-FP-ASSUMED. Reused natives by read-only path dependency (no private copies): warp_f2 (F2 body),
//! warp_f1 (F1, inside F2's flag path), bwb_libm (fmod, inside F2).

pub use warp_f2::{Args as F2Args, F2Error, Host, Image, Obj, Rec};
use warp_f1::Lattice;

/// CONTRACT-LIMIT: largest plane count n = [image] for which the real wrapper plus F2 return correctly in the emulator.
/// F2's temp row (rbp upward) overwrites its saved caller rbp for n >= 9; the wrapper then runs `mov rsp,rbp; pop rbp; ret`
/// with that rbp, so the call faults (MEASURED by the end-to-end T2 probes, t2/T2_RESULT.tsv). Beyond it `render` returns an error.
pub const CONTRACT_MAX_N: i32 = 8;

/// The primitive instance (rdi of the wrapper, 0xe0 bytes, vtable). Fields as read by .
#[derive(Clone, Debug)]
pub struct Prim {
    /// byte [P]: nonzero = constant coordinates (a1 = &P[0x6c], a2 = &P[0x70])
    pub const_coords: bool,
    /// f32 [P], [P]
    pub const_a1: f32,
    pub const_a2: f32,
    /// arrays [[P]] and [[P]] (used when const_coords is false; the wrapper adds 4*idx to the base)
    pub a1_arr: Vec<f32>,
    pub a2_arr: Vec<f32>,
    /// arrays [P], [P] (the wrapper adds 4*idx; written by slot 33)
    pub c1_arr: Vec<f32>,
    pub c2_arr: Vec<f32>,
    /// byte [P] (slot 33 writes it: ([P] != 0 and [P] == 0))
    pub flag: u8,
    /// i32 [P], [P]
    pub mx: i32,
    pub my: i32,
}

/// F2's frame object (the wrapper's rsi, F2's rdi) plus the one wrapper-level field [Q].
#[derive(Clone, Debug)]
pub struct Frame {
    pub obj: Obj,
    /// i32 [Q], sign extended (movsxd): element offset into the a1/a2/c1/c2 arrays
    pub idx: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RelError {
    /// n above CONTRACT_MAX_N: the original corrupts its frame (not emulated)
    ContractLimit { n: i32 },
    /// idx negative or beyond an array (the original would read outside the array)
    BadIndex,
    F2(F2Error),
}

/// What the wrapper hands F2 : slices are the arrays offset by the element index.
#[derive(Clone, Debug, PartialEq)]
pub struct WrapArgs {
    pub a1: Vec<f32>,
    pub a2: Vec<f32>,
    pub const_coords: bool,
    pub c1: Vec<f32>,
    pub c2: Vec<f32>,
    pub flag: i32,
    pub mx: i32,
    pub my: i32,
}

fn tail(v: &[f32], idx: i32) -> Result<Vec<f32>, RelError> {
    let i = usize::try_from(idx).map_err(|_| RelError::BadIndex)?;
    if i > v.len() {
        return Err(RelError::BadIndex);
    }
    Ok(v[i..].to_vec())
}

/// The wrapper's argument building, no pixel work.
pub fn wrapper_args(p: &Prim, idx: i32) -> Result<WrapArgs, RelError> {
    // const_coords picks &P[0x6c], &P[0x70]; else the base from [P] plus 4*idx
    let (a1, a2) = if p.const_coords {
        (vec![p.const_a1], vec![p.const_a2])
    } else {
        (tail(&p.a1_arr, idx)?, tail(&p.a2_arr, idx)?)
    };
    Ok(WrapArgs {
        a1,
        a2,
        const_coords: p.const_coords,
        c1: tail(&p.c1_arr, idx)?, // 
        c2: tail(&p.c2_arr, idx)?, // 
        flag: p.flag as i32,       // movzx eax, byte [P] 
        mx: p.mx,                  // [P] 
        my: p.my,                  // [P] 
    })
}

/// S3 style render entry: slot 53 of class warp_rel, host services explicit. `img` is [P] (the planar image object),
/// `out` the plane array (one Vec per plane, count entries each). Returns F2's call record.
pub fn render(p: &Prim, fr: &Frame, img: &mut Image, lat: &Lattice, out: &mut [Vec<f32>]) -> Result<Vec<Rec>, RelError> {
    let n = img.f1.n;
    if n > CONTRACT_MAX_N {
        return Err(RelError::ContractLimit { n });
    }
    let w = wrapper_args(p, fr.idx)?;
    let a = F2Args { a1: &w.a1, a2: &w.a2, const_coords: w.const_coords, c1: &w.c1, c2: &w.c2, flag: w.flag, mx: w.mx, my: w.my };
    warp_f2::run(&fr.obj, img, lat, &a, out, warp_f2::TEMP_PHASE_BYTES).map_err(RelError::F2)
}

#[derive(Clone, Debug)]
pub struct Assumption {
    pub id: &'static str,
    pub basis: &'static str,
    pub label: &'static str,
}

pub fn assumptions() -> Vec<Assumption> {
    vec![
        Assumption { id: "WR1-A1", basis: "ROUTE AVX2-ONLY: F2 dispatcher mask test 0x9d97ff selects; the SSE twin and tier are not ported (inherited from FX2-A1)", label: "MEASURED listing, ROUTE AVX2-ONLY" },
        Assumption { id: "WR1-A2", basis: "slot 33 (shared with class warp) writes P, P and P before slot 53 and calls prefill; here those P fields and the image object [P] are explicit inputs, the prefill callback is not rebuilt", label: "HOST-HOOKED (explicit inputs), NOT REBUILT: slot 33 chain" },
        Assumption { id: "WR1-A3", basis: "image channel count n = [[P]]: no store to P exists in the warp_rel constructor (base) or in slots 3,6,22,23,33,38,40,49,53,54,55,56,57 (listing read), so the binder is outside the class; n range of real callers UNKNOWN. Image constructor immediate n: 0, 1 and 3 (NTRACE.md); the port refuses n > 8", label: "UNKNOWN range; CONTRACT-LIMIT n <= 8 (measured by the end-to-end probes)" },
        Assumption { id: "WR1-A4", basis: "wrapper reads only P and Q fields listed in lib.rs; element index [Q] is an explicit input; the a1/a2/c1/c2 arrays are explicit slices (no pointer values are read by the port)", label: "ALIGN-INDEPENDENT" },
        Assumption { id: "WR1-A5", basis: "F2's inherited stand-ins: CPU word 0x9d97ff (route selector, PARAM-UNCAPTURED 1), assert helper and empty-image fills recorded only (CALLEE-MODELLED); fmod and F1 run as real code in the emulator and are the finished natives in Rust (no hook on fmod in this T2)", label: "inherited FX2-A2" },
        Assumption { id: "WR1-A6", basis: "MXCSR round to nearest, no FTZ, no DAZ", label: "HOST-FP-ASSUMED" },
    ]
}

/// Class summary (vtable, vtables\class_vtable_pairs.tsv row warp_rel), slot by slot at flag 0.
pub fn class_summary() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        ("33", " (416 B, shared with class warp)", "binder/prefill: writes P, calls (REC-PARITY by), NOT REBUILT here; must run before slot 53"),
        ("38", " (96 B)", "predicate (returns 0 or -1 from tests), no pixels"),
        ("40", " (192 B)", "bounds/extent query (or default), no pixels, NOT REBUILT"),
        ("53", " (160 B)", "CPU render entry: this port (wrapper plus F2, F1 on the flag path)"),
        ("54", " (144 B)", "GPU launch stub (calls), NOT REBUILT"),
        ("52", "-", "not in the class slot list (the base class slot 52 / process_short path is not warp_rel's own)"),
    ]
}
