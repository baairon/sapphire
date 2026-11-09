//! CPU-feature dispatchers (0x454b90 for the render body, 0x445a90 for the weight builder, 0xb7d170 for the sine).
//! The feature word is read through the pointer in the image cell at 0x1a98da0 (listing offset space).

/// Bits the AVX route requires (and 0x9d97ff, compare 0x9d97ff).
pub const AVX_MASK: u64 = 0x9d97ff;

/// Target chosen by a dispatcher.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Target {
    /// AVX body (0x454be0 / 0x445ae0).
    Avx,
    /// SSE body (0x455fd0 / 0x445bf0).
    Sse,
    /// Neither condition holds: the original calls the CPU init (0x136eab0) and loops back to the test.
    CpuInit,
}

/// Dispatcher 0x454b90 and 0x445a90: `(w & 0x9d97ff) == 0x9d97ff` selects the AVX body, else `w & 1 != 0` the SSE body.
pub fn select_body(w: u64) -> Target {
    if w & AVX_MASK == AVX_MASK {
        Target::Avx
    } else if w & 1 != 0 {
        Target::Sse
    } else {
        Target::CpuInit
    }
}

/// Dispatcher 0xb7d170 (sine): `(w & 0xff) == 0xff` selects 0xb7cc60, else `w & 1 != 0` selects 0xb7cee0, else CPU init.
pub fn select_sine(w: u64) -> Target {
    if w & 0xff == 0xff {
        Target::Avx
    } else if w & 1 != 0 {
        Target::Sse
    } else {
        Target::CpuInit
    }
}
