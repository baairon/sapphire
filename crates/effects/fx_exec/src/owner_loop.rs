//! INT17 step s11_prerender: byte copy of wave_s9/pdv3_owner_rest/owner_arm/owner_arm.rs below this line (PDV3 PATCH 1, W409 (b)).
//! PDV2: the owner arm of the original plugin ported as one pure function (W371 (c)).
//!
//! Sources (neutral_1 listing addresses; every statement labelled in ../README.md):
//!   * definition flag: sets bit 0x20 of [[owner]] for the token "dynamic" (cell);
//!      allocates the object stored at [owner] only when that bit is set.
//!   * setup arm: the parameter loop of the setup functions (and its siblings), ported from the
//!      copy (the SMART_PRE_RENDER function), whose arm events equal 's.
//!   * : ctx (mode byte), ctx, ctx, ctx.
//!   * negotiation twins : bit 2 of [owner].
//! No constants are chosen here: every input is a plugin result or a recorded host answer.

/// The four owner cells written by the arm plus bit 3 of the definition byte [[owner]].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OwnerCells {
    pub ae32_ready_c: u32,
    pub ae32_ready_d: u32,
    pub late_out_c: u32,
    pub late_out_d: u32,
    pub bit3: bool,
}

/// A program parameter as the loop sees it: [param] name, [param field] kind, and the integer
/// cvttss2si(value + 0.5f32) the code derives from the checked-out value (see `param_int`).
#[derive(Clone, Debug)]
pub struct ParamView {
    pub name: String,
    pub kind: u32,
    pub value: f32,
}

/// cvttss2si(value + 0.5f) as the code computes it (the constant is 0.5f,
/// MEASURED in place). Out-of-range values give 0x80000000 on hardware; Rust `as` saturates, so those are refused.
pub fn param_int(value: f32) -> Option<i32> {
    let x = value + 0.5f32;
    // INT18 (W414 (c) step 7): -2147483648.0f32 is in range for cvttss2si (it gives 0x80000000 as a value, not as the indefinite); the
    // earlier bound `x <= -2147483649.0f32` rounded to -2^31 in f32 and refused it
    if x.is_nan() || x >= 2147483648.0f32 || x < -2147483648.0f32 {
        None
    } else {
        Some(x as i32)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArmError {
    /// the value of an opacity or output_opacity parameter is NaN or out of the int range (hardware gives 0x80000000)
    ValueOutOfRange,
}

/// The setup arm. `def_dynamic` is `[owner] != 0bit2` is bit 2 of `[owner]`.
pub fn setup_arm(def_dynamic: bool, bit2: bool, params: &[ParamView]) -> Result<OwnerCells, ArmError> {
    // prelude (in)
    let mut c = if !def_dynamic {
        //  arm : [owner cell]=2, [owner cell]=0, bit3 set, [owner cell]=0, [owner cell]=0
        OwnerCells { ae32_ready_c: 2, ae32_ready_d: 0, late_out_c: 0, late_out_d: 0, bit3: true }
    } else if !bit2 {
        OwnerCells { ae32_ready_c: 1, ae32_ready_d: 0, late_out_c: 1, late_out_d: 1, bit3: false }
    } else {
        OwnerCells { ae32_ready_c: 1, ae32_ready_d: 0, late_out_c: 0, late_out_d: 0, bit3: true }
    };
    // loop over the program's parameters in order 
    for p in params {
        let opac = p.kind == 4 && (p.name == "opacity" || p.name == "input_opacity");
        if opac {
            // 
            let v = param_int(p.value).ok_or(ArmError::ValueOutOfRange)?;
            c.ae32_ready_c = v as u32;
            if v == 0 {
                c.ae32_ready_d = 3;
                c.bit3 = true;
            } else if v == 1 && !bit2 {
                c.late_out_c = 1;
                c.late_out_d = 1;
                c.bit3 = false;
            }
        } else if p.name == "output_opacity" {
            //  (any kind, including a kind 4 parameter with another name)
            let v = param_int(p.value).ok_or(ArmError::ValueOutOfRange)?;
            if v == 0 {
                c.ae32_ready_d = 3;
            } else {
                c.ae32_ready_d = 0;
            }
            if !bit2 {
                c.late_out_d = 0;
            }
        }
    }
    Ok(c)
}

/// The cells writes into the context (+ the mode byte). `prog_mode_word` is [[owner]],
/// `ctx_mode_cell` is [ctx], `dest_desc_mode` is [[ctx]].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CtxCells {
    pub ctx_cells_a: u8,
    pub ctx_cells_b: u32,
    pub ctx_cells_c: u32,
    pub ctx_cells_d: u32,
}

pub fn ctx_init(o: &OwnerCells, prog_mode_word: u32, ctx_mode_cell: u32, dest_desc_mode: u32) -> CtxCells {
    // r8d = [owner] if non-zero, else [prog]; the mode byte is r8b.
    let r8 = if o.ae32_ready_d != 0 { o.ae32_ready_d } else { prog_mode_word };
    let mut mode = (r8 & 0xff) as u8;
    if mode == 2 {
        // every path rejoins (the cells below are always written)
        if ctx_mode_cell == 0 {
            mode = 3;
        } else if dest_desc_mode == 3 {
            mode = 1;
        }
    }
    CtxCells { ctx_cells_a: mode, ctx_cells_b: o.late_out_c, ctx_cells_c: o.bit3 as u32, ctx_cells_d: o.late_out_d }
}

/// The host side of the Image Format Negotiation block (twin).
#[derive(Clone, Debug)]
pub struct Negotiation {
    /// the host kind cell (1 on the claim host)
    pub kind: u32,
    /// the cell; initial value 1 and no writer in the text
    pub gate_cell_a: u32,
    /// [ctx] non-null and [[ctx]] non-null (the acquire function exists)
    pub acquire_present: bool,
    /// the suite pointer written by the acquire call is non-null
    pub suite_acquired: bool,
    /// return of the first suite call [suite] (writes the answer record at rbp)
    pub ret1: u32,
    /// the answer record: byte 0 (flag), the C string at byte 1, the qword at byte 0x10
    pub flag: u8,
    pub fmt: Vec<u8>,
    pub mode: u64,
    /// return of the second suite call [suite field]([ctx])
    pub ret2: u32,
}

/// New value of [owner] bits 0x4 and 0x8 (the other bits are left alone); `prog_opacity_bit` is bit 0x40 of
/// [[owner]] (set by a parameter named opacity or input_opacity). None = no suite block ran.
pub fn negotiate(prev_cell_a: u32, prog_opacity_bit: bool, n: Option<&Negotiation>) -> u32 {
    let mut f = prev_cell_a;
    let n = match n {
        Some(n) => n,
        None => return f,
    };
    // gate : (bit 0x40 set or kind == 0xa) and != 0
    if !(prog_opacity_bit || n.kind == 0xa) || n.gate_cell_a == 0 {
        return f;
    }
    // 
    if n.kind == 3 || !n.acquire_present || !n.suite_acquired || n.ret1 != 0 {
        return f;
    }
    let mut ecx = false;
    if n.fmt == b"BGRA" && n.mode == 0xffff {
        f |= 0x8;
        ecx = true;
    }
    if n.fmt == b"ARGB" && n.mode == 0x8000 {
        ecx = true;
    }
    if !prog_opacity_bit && n.flag != 0 {
        return f & !0xc;
    }
    if !ecx {
        return f;
    }
    if n.ret2 != 0 {
        return f & !0xc;
    }
    if n.flag == 0 {
        return f;
    }
    f | 0x4
}

/// Inputs of the later pre-render stage (PDV3, only; PRERENDER.md).
/// `rect_flag[k]` is the byte at [rbp+10k-] (the empty flag of the k-th 10-byte rectangle record at rbp),
/// `a8[k]` the dword at [rbp+4k-0xa8] (a byte of the k-th host checkout result, written or zeroed).
#[derive(Clone, Debug)]
pub struct LateInputs {
    pub ae32_ready_b: u32,
    pub ae32_ready_a: u8,
    pub prog_mode_word: u32,
    /// [prog] as the signed byte movsx reads it
    pub late_inputs_a: i8,
    pub rect_flag: Vec<u8>,
    pub a8: Vec<u32>,
}

/// The flag the stage keeps in [rbp] (stored later as byte [out] and dword [out_ptr]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LateOut {
    pub ae32_ready_d: u32,
    pub rbp_cell_a: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LateError {
    /// [prog] indexes outside the recorded arrays (negative or past the end): the code would read other stack words
    IndexOutOfRange,
}

/// The stage : [owner] becomes 3 (and [rbp] = 1) when it is already 3, or when
/// [prog] is 2 and (the rectangle flag at index [prog] is 1, or the checkout byte there is non-zero),
/// or when [owner] is 3 and ([prog] is 3, or it is 2 and the checkout byte is non-zero, or it is 2 and the
/// rectangle flag test above holds). Otherwise [rbp] = 0 and the cell is unchanged.
pub fn later_stage(i: &LateInputs) -> Result<LateOut, LateError> {
    let keep = LateOut { ae32_ready_d: i.ae32_ready_b, rbp_cell_a: 0 };
    let set = LateOut { ae32_ready_d: 3, rbp_cell_a: 1 };
    // cmp [owner cell],3; je 
    if i.ae32_ready_b == 3 {
        return Ok(set);
    }
    let idx = |i_: &LateInputs| -> Result<usize, LateError> {
        let k = i_.late_inputs_a;
        if k < 0 || (k as usize) >= i_.a8.len() || (k as usize) >= i_.rect_flag.len() {
            Err(LateError::IndexOutOfRange)
        } else {
            Ok(k as usize)
        }
    };
    let eax = i.prog_mode_word;
    if i.ae32_ready_a == 3 {
        // 
        if eax == 3 {
            return Ok(set);
        }
        if eax != 2 {
            return Ok(keep);
        }
        let k = idx(i)?;
        if i.a8[k] != 0 {
            return Ok(set);
        }
        // jmp : the same test as the arm without reloading rcx
        return Ok(if i.rect_flag[k] == 1 || i.a8[k] != 0 { set } else { keep });
    }
    if eax == 2 {
        // rect flag == 1 stores; else a8 == 0 does not store, a8 != 0 stores
        let k = idx(i)?;
        if i.rect_flag[k] == 1 {
            return Ok(set);
        }
        return Ok(if i.a8[k] == 0 { keep } else { set });
    }
    Ok(keep)
}

/// The derived postdivide cell and variant for a recorded style: returns (owner cells, ctx cells).
pub fn derive(def_dynamic: bool, bit2: bool, params: &[ParamView], prog_mode_word: u32, ctx_mode_cell: u32, dest_desc_mode: u32) -> Result<(OwnerCells, CtxCells), ArmError> {
    let o = setup_arm(def_dynamic, bit2, params)?;
    let c = ctx_init(&o, prog_mode_word, ctx_mode_cell, dest_desc_mode);
    Ok((o, c))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(name: &str, kind: u32, value: f32) -> ParamView {
        ParamView { name: name.to_string(), kind, value }
    }

    /// Recorded rows: run, kind of style, the four owner cells, late_out_a, late_out_b (-1 unknown), ctx_postdiv_cell (-1 unknown), opacity value (-1 none)
    static ROWS: effects_rtdata::Table = effects_rtdata::Table::new("fx_exec/src/RECORDED_CELLS.tsv");

    #[test]
    fn every_recorded_owner_row() {
    if !effects_rtdata::have_tables() {
        return;
    }
        let mut n = 0;
        let mut ctx_postdiv_checked = 0;
        for ln in ROWS.lines().skip(1) {
            let f: Vec<&str> = ln.split('\t').collect();
            let run = f[0];
            let rec: Vec<u32> = f[2..6].iter().map(|x| x.parse().unwrap()).collect();
            let late_out_a: i64 = f[6].parse().unwrap();
            let late_out_b: i64 = f[7].parse().unwrap();
            let ctx_cells_d: i64 = f[8].parse().unwrap();
            let opac: f32 = f[9].parse().unwrap();
            // def_dynamic: derived (no style sets the token); where the 13e0 cell was recorded it must agree
            let def_dynamic = false;
            if late_out_b >= 0 {
                assert_eq!((late_out_b != 0), def_dynamic, "{}: recorded late_out_b", run);
            }
            let mut params = vec![];
            if opac >= 0.0 {
                params.push(p("opacity", 4, opac));
            }
            let (o, c) = derive(def_dynamic, late_out_a >= 0 && (late_out_a & 4) != 0, &params, 1, 1, 0).unwrap();
            assert_eq!(vec![o.ae32_ready_c, o.ae32_ready_d, o.late_out_c, o.late_out_d], rec, "{}: owner cells", run);
            if ctx_cells_d >= 0 {
                assert_eq!(c.ctx_cells_d as i64, ctx_cells_d, "{}: ctx_postdiv_cell", run);
                ctx_postdiv_checked += 1;
            }
            n += 1;
        }
        assert!(n >= 51, "rows {}", n);
        assert!(ctx_postdiv_checked >= 4);
    }

    /// TWR1 (W722): the four recorded (opacity, output opacity) loop pairs of S_TimeWarpRGB give the recorded owner tuples (the owner cells)
    #[test]
    fn time_warp_opacity_output_opacity_pairs() {
        for (op, out, want) in [(1.0f32, 0.0f32, [1, 3, 1, 0]), (0.0, 0.0, [0, 3, 0, 0]), (2.0, 0.0, [2, 3, 0, 0]), (1.0, 1.0, [1, 0, 1, 0])] {
            let o = setup_arm(false, false, &[p("opacity", 4, op), p("output_opacity", 4, out)]).unwrap();
            assert_eq!([o.ae32_ready_c, o.ae32_ready_d, o.late_out_c, o.late_out_d], want, "opacity {} output_opacity {}", op, out);
        }
        // the opacity-only list (the pre-TWR1 fx_run list) leaves [owner] 0 on the first three pairs: the defect the pairs discriminate
        let o = setup_arm(false, false, &[p("opacity", 4, 1.0)]).unwrap();
        assert_eq!([o.ae32_ready_c, o.ae32_ready_d, o.late_out_c, o.late_out_d], [1, 0, 1, 1]);
    }

    #[test]
    fn arm_table() {
        // no opacity parameter, no dynamic: the arm in every bit 2 state
        let a = setup_arm(false, false, &[]).unwrap();
        assert_eq!(a, OwnerCells { ae32_ready_c: 2, ae32_ready_d: 0, late_out_c: 0, late_out_d: 0, bit3: true });
        // opacity popup = 1 (Normal), bit 2 clear: v = 1
        let b = setup_arm(false, false, &[p("opacity", 4, 1.0)]).unwrap();
        assert_eq!((b.ae32_ready_c, b.late_out_c, b.late_out_d, b.bit3), (1, 1, 1, false));
        // bit 2 set: nothing more
        let b2 = setup_arm(false, true, &[p("opacity", 4, 1.0)]).unwrap();
        assert_eq!((b2.ae32_ready_c, b2.late_out_c, b2.late_out_d, b2.bit3), (1, 0, 0, true));
        // opacity 0 (All_Opaque): 1358 = 3, v = 0
        let z = setup_arm(false, false, &[p("opacity", 4, 0.0)]).unwrap();
        assert_eq!((z.ae32_ready_c, z.ae32_ready_d, z.late_out_c, z.late_out_d, z.bit3), (0, 3, 0, 0, true));
        // opacity 2 (As_Premult): 1354 = 2, v stays 0
        let t = setup_arm(false, false, &[p("opacity", 4, 2.0)]).unwrap();
        assert_eq!((t.ae32_ready_c, t.late_out_c, t.late_out_d), (2, 0, 0));
        // kind 5 named opacity is not the arm
        let k = setup_arm(false, false, &[p("opacity", 5, 1.0)]).unwrap();
        assert_eq!(k, a);
        // dynamic: ordinary arm
        let d = setup_arm(true, false, &[]).unwrap();
        assert_eq!((d.ae32_ready_c, d.late_out_c, d.late_out_d, d.bit3), (1, 1, 1, false));
        let d2 = setup_arm(true, true, &[]).unwrap();
        assert_eq!((d2.ae32_ready_c, d2.late_out_c, d2.late_out_d, d2.bit3), (1, 0, 0, true));
        // NaN and out-of-range refused
        assert_eq!(setup_arm(false, false, &[p("opacity", 4, f32::NAN)]), Err(ArmError::ValueOutOfRange));
        assert_eq!(setup_arm(false, false, &[p("opacity", 4, 3.0e9)]), Err(ArmError::ValueOutOfRange));
        // rounding: 0.49 -> 0, 0.5 -> 1, 1.49 -> 1, 1.5 -> 2, -0.4 -> 0 (truncation toward zero of value + 0.5)
        assert_eq!(param_int(0.49), Some(0));
        assert_eq!(param_int(0.5), Some(1));
        assert_eq!(param_int(1.49), Some(1));
        assert_eq!(param_int(1.5), Some(2));
        assert_eq!(param_int(-0.4), Some(0));
        assert_eq!(param_int(-0.6), Some(0));
        assert_eq!(param_int(-1.6), Some(-1));
    }

    #[test]
    fn ctx_table() {
        let o = setup_arm(false, false, &[]).unwrap();
        let c = ctx_init(&o, 1, 1, 0);
        assert_eq!((c.ctx_cells_a, c.ctx_cells_b, c.ctx_cells_c, c.ctx_cells_d), (1, 0, 1, 0)); // variant B, no postdivide
        let o = setup_arm(false, false, &[p("opacity", 4, 1.0)]).unwrap();
        let c = ctx_init(&o, 1, 1, 0);
        assert_eq!((c.ctx_cells_b, c.ctx_cells_c, c.ctx_cells_d), (1, 0, 1)); // variant A, postdivide
        // mode byte 2 paths rejoin the writes of 76c/770/774
        let c = ctx_init(&o, 2, 0, 0);
        assert_eq!((c.ctx_cells_a, c.ctx_cells_b, c.ctx_cells_c, c.ctx_cells_d), (3, 1, 0, 1));
        let c = ctx_init(&o, 2, 1, 3);
        assert_eq!((c.ctx_cells_a, c.ctx_cells_d), (1, 1));
        let c = ctx_init(&o, 2, 1, 0);
        assert_eq!(c.ctx_cells_a, 2);
    }

    fn neg(flag: u8, fmt: &[u8], mode: u64) -> Negotiation {
        Negotiation { kind: 1, gate_cell_a: 1, acquire_present: true, suite_acquired: true, ret1: 0, flag, fmt: fmt.to_vec(), mode, ret2: 0 }
    }

    #[test]
    fn negotiation_table() {
        // no block (no opacity parameter, kind != 0xa): unchanged
        assert_eq!(negotiate(0x10, false, Some(&neg(1, b"BGRA", 0xffff))), 0x10);
        // opacity parameter present, flag set, BGRA with 0xffff: bits 0x8 and 0x4 set
        assert_eq!(negotiate(0x10, true, Some(&neg(1, b"BGRA", 0xffff))), 0x10 | 0xc);
        // ARGB with 0x8000: bit 0x4 only
        assert_eq!(negotiate(0x10, true, Some(&neg(1, b"ARGB", 0x8000))), 0x10 | 0x4);
        // flag clear: nothing
        assert_eq!(negotiate(0x10, true, Some(&neg(0, b"BGRA", 0xffff))), 0x10 | 0x8);
        // mismatching mode: ecx clear, unchanged
        assert_eq!(negotiate(0x12, true, Some(&neg(1, b"BGRA", 0x1))), 0x12);
        // second call fails: both bits cleared
        let mut n = neg(1, b"ARGB", 0x8000);
        n.ret2 = 5;
        assert_eq!(negotiate(0x1c, true, Some(&n)), 0x10);
        // first call fails: unchanged
        let mut n = neg(1, b"ARGB", 0x8000);
        n.ret1 = 1;
        assert_eq!(negotiate(0x10, true, Some(&n)), 0x10);
        // suite absent (plain host): unchanged
        let mut n = neg(1, b"ARGB", 0x8000);
        n.suite_acquired = false;
        assert_eq!(negotiate(0x10, true, Some(&n)), 0x10);
        // the claim-host stand-in (harness answer: every output untouched, stale zero record): bit 2 stays clear
        assert_eq!(negotiate(0x10, true, Some(&neg(0, b"", 0))), 0x10);
        // kind 3 refused
        let mut n = neg(1, b"ARGB", 0x8000);
        n.kind = 3;
        assert_eq!(negotiate(0x10, true, Some(&n)), 0x10);
    }

    fn late(ae32_ready_b: u32, ae32_ready_a: u8, g3_in_a: u32, late_inputs_a: i8, rect: [u8; 5], a8: [u32; 5]) -> LateInputs {
        LateInputs { ae32_ready_b, ae32_ready_a, prog_mode_word: g3_in_a, late_inputs_a, rect_flag: rect.to_vec(), a8: a8.to_vec() }
    }

    #[test]
    fn later_stage_recorded_cells() {
        // PDV3 records (pvHS, Hue_Sat_Bright_Autogen, two pre-renders): ae32_ready_b 0, prog_mode_word 2, ae32_ready_a 4, late_inputs_a 0,
        // rect flag[0] 0, a8[0] 0: no store, the recorded [owner cell] stays 0 at both entries (MEASURED).
        let a = later_stage(&late(0, 4, 2, 0, [0, 1, 86, 0, 0], [0, 0, 2146420336, 0, 275190146])).unwrap();
        assert_eq!(a, LateOut { ae32_ready_d: 0, rbp_cell_a: 0 });
        let b = later_stage(&late(0, 4, 2, 0, [0, 1, 0, 0, 0], [0, 0, 1621312320, 0, 1621178880])).unwrap();
        assert_eq!(b, LateOut { ae32_ready_d: 0, rbp_cell_a: 0 });
        // PDV3 records (pvBL, Blur_Color, prog_mode_word 1): never stores
        let c = later_stage(&late(0, 4, 1, 0, [0, 1, 64, 0, 0], [0, 0, 1624846784, 0, 1624802816])).unwrap();
        assert_eq!(c, LateOut { ae32_ready_d: 0, rbp_cell_a: 0 });
    }

    #[test]
    fn later_stage_branches() {
        let z = [0u32; 5];
        let r0 = [0u8; 5];
        // already 3 stays 3 and sets the flag
        assert_eq!(later_stage(&late(3, 4, 1, 0, r0, z)).unwrap(), LateOut { ae32_ready_d: 3, rbp_cell_a: 1 });
        // prog_mode_word 2, rect flag 1 at index 0: store
        assert_eq!(later_stage(&late(0, 4, 2, 0, [1, 0, 0, 0, 0], z)).unwrap().ae32_ready_d, 3);
        // prog_mode_word 2, checkout byte non-zero at the index: store
        assert_eq!(later_stage(&late(0, 4, 2, 1, r0, [0, 7, 0, 0, 0])).unwrap().ae32_ready_d, 3);
        // prog_mode_word 2, both clear: no store
        assert_eq!(later_stage(&late(0, 4, 2, 1, r0, z)).unwrap().ae32_ready_d, 0);
        // the index is the signed byte [prog]: negative or past the arrays is refused
        assert_eq!(later_stage(&late(0, 4, 2, -1, r0, z)), Err(LateError::IndexOutOfRange));
        assert_eq!(later_stage(&late(0, 4, 2, 5, r0, z)), Err(LateError::IndexOutOfRange));
        // prog_mode_word 3 stores only when [owner] is 3
        assert_eq!(later_stage(&late(0, 4, 3, 0, r0, z)).unwrap().ae32_ready_d, 0);
        assert_eq!(later_stage(&late(0, 3, 3, 0, r0, z)).unwrap().ae32_ready_d, 3);
        // ae32_ready_a 3 with prog_mode_word 2: checkout byte non-zero stores; else the rect flag decides
        assert_eq!(later_stage(&late(0, 3, 2, 0, r0, [1, 0, 0, 0, 0])).unwrap().ae32_ready_d, 3);
        assert_eq!(later_stage(&late(0, 3, 2, 0, [1, 0, 0, 0, 0], z)).unwrap().ae32_ready_d, 3);
        assert_eq!(later_stage(&late(0, 3, 2, 0, r0, z)).unwrap().ae32_ready_d, 0);
        // ae32_ready_a 3 with prog_mode_word 1: no store
        assert_eq!(later_stage(&late(0, 3, 1, 0, r0, z)).unwrap().ae32_ready_d, 0);
    }

    /// The mode byte seen by the second call with the stage in place: a stored 3 turns ctx into 3
    /// and the cells stay (this is the dependence PDV2's mutant `ae32_ready_d != 0` to `> 1` probes).
    #[test]
    fn later_stage_feeds_ctx_init() {
        let mut o = setup_arm(false, false, &[]).unwrap();
        let l = later_stage(&late(0, 4, 2, 0, [1, 0, 0, 0, 0], [0; 5])).unwrap();
        o.ae32_ready_d = l.ae32_ready_d;
        let c = ctx_init(&o, 2, 1, 0);
        assert_eq!(c.ctx_cells_a, 3);
        // without the stage the mode byte would come from prog_mode_word (2, adjusted by ctx_mode_cell / dest_desc_mode)
        let o0 = setup_arm(false, false, &[]).unwrap();
        assert_eq!(ctx_init(&o0, 2, 1, 0).ctx_cells_a, 2);
        // [owner cell] = 1 and [owner cell] = 3 both are non-zero and select the cell as the mode: `!= 0` and `> 1` differ at 1 (not reachable: [owner cell] is 0 or 3)
        let mut o1 = o0;
        o1.ae32_ready_d = 1;
        assert_eq!(ctx_init(&o1, 2, 1, 0).ctx_cells_a, 1);
    }
}
