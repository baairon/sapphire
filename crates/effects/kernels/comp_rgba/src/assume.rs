//! Every stand-in, route choice and host fact of the comp_rgba port, with its tier. Prefix -.

#[derive(Clone, Debug)]
pub struct Assumption {
    pub id: &'static str,
    pub tier: &'static str,
    pub what: &'static str,
}

pub fn assumptions() -> Vec<Assumption> {
    vec![
        Assumption { id: "CR1-A1", tier: "CAPTURED", what: "ONE f32 1.0 (bytes 0000803f): T >= 1 test and 1 - T in the float routes" },
        Assumption { id: "CR1-A2", tier: "CAPTURED", what: "ONE f64 1.0 (bytes 000000000000f03f): the const-B double routes (local (1,1), (3,2);)" },
        Assumption { id: "CR1-A3", tier: "ROUTE", what: "AVX2 route only: feature word cell is captured, the word behind it is served as 0x9d97ff; the SSE twins are not ported" },
        Assumption { id: "CR1-A4", tier: "HOST-FP-ASSUMED", what: "MXCSR round to nearest, no FTZ, no DAZ; the emulator follows the SDM, not silicon" },
        Assumption { id: "CR1-A5", tier: "CALLEE-MODELLED / PAD-UNVERIFIED", what: "combo (4,3): the body calls the host plane service (mode 1, chans 0..2) and the fill (value 1.0); the T2 compares the call record only. apply_pad runs them through the finished plane_service crate with an explicit block (the class virtual's block is uncaptured) and re-expresses the 1.0 fill as a constant flat fill" },
        Assumption { id: "CR1-A6", tier: "HOST-HOOKED", what: "unimplemented (ca,cb): class virtual [vtable] (returns handled or not) and the log are host services, record only" },
        Assumption { id: "CR1-A7", tier: "INFERRED", what: "ca is the count at this and cb the count at this; that the host fills them from the operand types of the two inputs, and the byte flags at this / this as constant-operand markers, is INFERRED from the body (the slot 60 / 84 binding was not run)" },
        Assumption { id: "CR1-A8", tier: "CONTRACT", what: "operand records non-null when read, planes at least n floats, output planes distinct from inputs (the original reads then writes each pixel but aliasing across channels is not modelled)" },
        Assumption { id: "CR1-A9", tier: "SEMANTIC-NONE", what: "no uncaptured data cell is read on the AVX2 route (ASSUMED-SEMANTIC 0, PARAM-UNCAPTURED 0)" },
    ]
}
