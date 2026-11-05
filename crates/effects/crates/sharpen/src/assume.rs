//! Assumption table (S7).
#[derive(Clone, Debug)]
pub struct Assumption {
    pub id: &'static str,
    pub what: &'static str,
    pub label: &'static str,
}

pub fn assumptions() -> Vec<Assumption> {
    vec![
        Assumption { id: "SP-A1", what: "render flag 0 only; flag != 0 is a host serializer path and is not rebuilt", label: "INFERRED (SPEC section 5)" },
        Assumption { id: "SP-A2", what: "combine job pointer cell is not captured: job = AVX2 body ", label: "ASSUMED" },
        Assumption { id: "SP-A3", what: "ctx[0x40] count = roi width * height (blur writes w*h to the same global)", label: "ASSUMED" },
        Assumption { id: "SP-A4", what: "ctx[8] == 0: roi record {w, h, c = w, d = 0, e = 4}; dense planes (stride == width)", label: "ASSUMED" },
        Assumption { id: "SP-A5", what: "plane count, input count and gains are caller inputs; in_count == plane count here", label: "ASSUMED" },
        Assumption { id: "SP-A6", what: "alias scratch plane is internal (blur and combine are the same arithmetic)", label: "INFERRED" },
        Assumption { id: "B-A1", what: "cancel polls /10 return 0 (inherited from blur)", label: "ASSUMED" },
        Assumption { id: "B-A4", what: "blur scratch base phase; sharpen uses blur::DEFAULT_BASE_PHASE (see README)", label: "ASSUMED" },
        Assumption { id: "B-A6", what: "single worker: partition gives start 0, count = total", label: "ASSUMED" },
        Assumption { id: "U05", what: "feature word 0x9d97ff (AVX2 + FMA bodies)", label: "ASSUMED" },
    ]
}
