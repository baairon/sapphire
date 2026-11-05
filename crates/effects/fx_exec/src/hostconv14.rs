//! INT9 (Batch 43 rules 1 to 4, W341 to W343): the After Effects 32-bpc source route ('ae32', HOST-PIXEL-FORMAT: memory A,R,G,B f32, 16 bytes
//! per pixel, descriptor format 14 in). The format-14 source readers are FMT3a's port (crate readers14: R4, R3, R1,
//! variant A with stack word 0; "R-PARITY and FLOOR-MET", W341). Which reader body runs is NOT chosen here: the caller passes the reader head the
//! counted run's head hooks recorded for each source; a head other than R4, R3 or R1 (the stack-word twins, variant B
//! variant-B stack-word, the code-13 source arm, or any other) is refused by name
//! (READER-BODY-NOT-PORTED: ported and floored by FMT3e first; it blocks END-TO-END).
//! LOOP-SHAPE-EQUIVALENT: the plain loop shape is used (block = false); the CPU-flag word is a recorded cell of the run, not an input here.
//! CONTRACT-LIMIT READER-COUNT-RANGE: a line above 2^31 - 16 pixels is refused by name.
//! TABLE-NULL-ON-ROUTE (W342 (d), W343): the curve table at is NULL on the route and the stack-word globals,
//!  are 0, so the code-13 curve writers (FMT3b) are off route and are not reachable from this module: a destination request on the
//! 'ae32' route with a nonzero stack word is refused as HOST-GATED. INT10: the code-14 destination bodies (DEST-ROUTE, FMT3c's crate c14,
//! R-PARITY (FLOOR PENDING)) answer convert-to-dest on the 'ae32' route (`ae32_dest`); the format-16 writer never stands in.

use crate::types::{ExecError, Image};
pub use crate::judge_keys::{head_name, R1_HEAD, R3_HEAD, R4_HEAD, UNPORTED_HEADS, VB1_HEAD, VB3_HEAD, VB4_HEAD};
use readers14::{read1, read3, read4, ReadErr};
use readers14b::{gate_is_b, read1b, read3b, read4b};

/// Reader heads with a ported, floored body (FMT3a), by listing address.
/// The heads Batch 43 rule 1 lists that have no ported body here.
/// INT10 (FMT3e, Batch 44 rule 1): the variant-B stack-word-zero bodies, "R-PARITY (FLOOR PENDING second reader)" (FMT3e: 593,680 floats,
/// 0 differing; 20 equivalent survivors with one reader's proofs, RDR3 outstanding). Variant B does not multiply by alpha.

/// SVB1 (W744 (b)): the class-3 hand-over of the variant-B 4-plane head . The head reads four planes (read4b: R, G, B, then the alpha
/// lane); the class-3 body takes the three colour lanes, so the first three planes are handed over and the alpha lane is dropped. Only for a
/// source whose record head is VB4_HEAD; every other head keeps its plane count (a mismatch is refused by ae32_to_image).
pub fn class3_vb4_lanes(s: &Ae32Source, w: usize, h: usize) -> Result<Image, ExecError> {
    if s.reader_head != VB4_HEAD || s.planes != 4 {
        return Err(ExecError::Shape(format!("CLASS3-VB4-HANDOVER-REFUSED: head 0x{:x} with {} planes (only with 4 planes is handed over as 3 lanes)", s.reader_head, s.planes)));
    }
    // SVB1 mutants (build-time features, never shipped): read the path through R4 (alpha-weighted), or hand over the wrong lanes
    let src = if cfg!(feature = "svb1_r4_mut") { Ae32Source { words: s.words.clone(), reader_head: R4_HEAD, planes: 4 } } else { s.clone() };
    let im = ae32_to_image(&src, w, h)?;
    let mut pl = im.planes;
    if cfg!(feature = "svb1_lane_mut") {
        pl.remove(0);
    } else {
        pl.truncate(3);
    }
    Image::from_planes(pl)
}

/// The source gate's inputs (4 planes, 3, 1): variant B iff ([rsi] != 0 and [ctx] == 0) or [ctx] != 0.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SourceGate {
    /// [row]: writer not located (FMT3e), a recorded input (0 in 128 of 128 recorded gate hits)
    pub row_gate_cell: u32,
    pub ctx_gate_a: u32,
    pub ctx_gate_b: u32,
}

/// INT11 (Batch 46 rule 5, W371): the inputs of the owner arm, recorded at setup ("OWNER-ARM-RECORDED" until PDV2 derives them).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OwnerArm {
    /// [owner] != 0
    pub owner_arm_nonzero: bool,
    /// bit 2 of [owner] (set only by the Image Format Negotiation twins)
    pub owner_arm_bit2: bool,
    /// the mode-byte-2 exit: no setup arm runs and the constructor defaults stand 
    pub mode_byte_exit: bool,
    /// INT12 (Batch 47 rule 4, W380): the f32 bits of the checked-out record value of the program's kind-4 parameter named "opacity" or
    /// "input_opacity", None when the program registers none. A recorded input (OWNER-ARM-RECORDED).
    pub opacity_bits: Option<u32>,
    /// LYM1: the record value of the program's parameter named "output_opacity" (branch), None when the program has none (Time_Warp_RGB's layout `:extra`)
    pub output_opacity_bits: Option<u32>,
}

/// The three context cells copies from the owner: (ctx, ctx, ctx).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArmCells {
    pub ctx_gate_a: u32,
    pub ctx_gate_b: u32,
    pub ctx_postdiv_cell: u32,
}

/// INT11 (Batch 46 rule 5): every setup arm writes [owner] = [owner] = v and bit 3 of the definition byte = not v (listing
/// ), v = 1 iff [owner] != 0 and bit 2 of [owner] is clear; 
/// copies them to (ctx) = (v, not v, v). The mode-byte-2 exit leaves the constructor defaults (0, 1, 1).
/// So the source variant is B iff v = 0 and the postdivide primitive is inserted iff v = 1 (PDV1: iff ctx != 0).
///
/// INT12 (Batch 47 rule 4, W380): the setup body on the recorded routes (test) is followed by the parameter loop from
/// a kind-4 parameter "opacity" / "input_opacity" with m = trunc(0.5 + record) (0.5) gives m = 0: [owner] = 3
/// and bit 3 set; m = 1 with bit 2 of [owner] clear: v = 1 . Ported as PDV2's owner_loop::setup_arm;
///  copies (bit 3) to (ctx). Last store wins, in parameter order.
/// INT12 (PDV2 PATCH 4, coordinator note): there is no mode-byte-2 early exit (rejoins, so always writes the three
/// cells); an input claiming that exit is refused by name (MODE-BYTE2-EXIT-WITHDRAWN) instead of giving the constructor defaults.
pub fn try_owner_arm(a: &OwnerArm) -> Result<ArmCells, ExecError> {
    if a.mode_byte_exit {
        return Err(ExecError::Shape("MODE-BYTE2-EXIT-WITHDRAWN: rejoins; ctx are always written by (PDV2)".into()));
    }
    let params: Vec<crate::owner_loop::ParamView> = match a.opacity_bits {
        Some(b) => vec![crate::owner_loop::ParamView { name: "opacity".to_string(), kind: 4, value: f32::from_bits(b) }],
        None => vec![],
    };
    let mut params = params;
    if let Some(b) = a.output_opacity_bits {
        params.push(crate::owner_loop::ParamView { name: "output_opacity".to_string(), kind: 4, value: f32::from_bits(b) });
    }
    let o = crate::owner_loop::setup_arm(a.owner_arm_nonzero, a.owner_arm_bit2, &params)
        .map_err(|e| ExecError::Shape(format!("OWNER-ARM-VALUE-OUT-OF-RANGE: {:?} (opacity bits {:?})", e, a.opacity_bits)))?;
    Ok(ArmCells { ctx_gate_a: o.late_out_c, ctx_gate_b: o.bit3 as u32, ctx_postdiv_cell: o.late_out_d })
}

/// The owner arm (panics only on an opacity value outside the int range, which try_owner_arm refuses by name).
pub fn owner_arm(a: &OwnerArm) -> ArmCells {
    try_owner_arm(a).expect("owner arm")
}

/// INT11's arm without the Opacity loop, kept only so t25 can show the loop is needed (it fails on BCJ2's Blur_Color cells).
pub fn owner_arm_no_loop(a: &OwnerArm) -> ArmCells {
    if a.mode_byte_exit {
        return ArmCells { ctx_gate_a: 0, ctx_gate_b: 1, ctx_postdiv_cell: 1 };
    }
    let v = (a.owner_arm_nonzero && !a.owner_arm_bit2) as u32;
    ArmCells { ctx_gate_a: v, ctx_gate_b: 1 - v, ctx_postdiv_cell: v }
}

/// do_postdiv of the compile, derived by the owner arm ("POSTDIV-DERIVED"; the recorded cell ctx is a check).
pub fn derived_do_postdiv(a: &OwnerArm) -> bool {
    owner_arm(a).ctx_postdiv_cell != 0
}

impl SourceGate {
    /// The source gate from the owner arm's cells and the recorded [row] (INT11: replaces FMT3e PATCH 2's from_owner arms).
    pub fn from_arm(a: &OwnerArm, row_gate_cell: u32) -> SourceGate {
        let c = owner_arm(a);
        SourceGate { row_gate_cell, ctx_gate_a: c.ctx_gate_a, ctx_gate_b: c.ctx_gate_b }
    }
    /// FMT3e PATCH 2's two-input form, now through the owner arm (no mode-byte-2 exit).
    pub fn from_owner(owner_arm_nonzero: bool, owner_arm_bit2: bool, row_gate_cell: u32) -> SourceGate {
        SourceGate::from_arm(&OwnerArm { owner_arm_nonzero, owner_arm_bit2, mode_byte_exit: false, opacity_bits: None, output_opacity_bits: None }, row_gate_cell)
    }
}

/// The reader head the native selects for a plane count and gate (stack word 0 on the route). The head hooks' record is a check, never the input.
/// INT11 (Batch 46 rule 4, W370): the gate state (ctx, ctx) = (0, 0) makes the variant depend on [row], whose writer is
/// not located: refused by name (SOURCE-GATE-ROW58-UNRESOLVED), never defaulted. The owner arm never gives (0, 0).
pub fn derived_head(planes: usize, g: &SourceGate) -> Result<u64, ExecError> {
    if g.ctx_gate_a == 0 && g.ctx_gate_b == 0 {
        return Err(ExecError::Shape(format!("SOURCE-GATE-ROW58-UNRESOLVED: (ctx, ctx) = (0, 0) with [row] = {} (its writer is not located)", g.row_gate_cell)));
    }
    let b = gate_is_b(g.row_gate_cell, g.ctx_gate_a, g.ctx_gate_b);
    Ok(match (planes, b) {
        (4, false) => R4_HEAD,
        (3, false) => R3_HEAD,
        (1, false) => R1_HEAD,
        (4, true) => VB4_HEAD,
        (3, true) => VB3_HEAD,
        (1, true) => VB1_HEAD,
        (p, _) => return Err(ExecError::Shape(format!("no format-14 reader for {} planes", p))),
    })
}

/// A source with the derived head; a recorded head that differs is a finding (SOURCE-VARIANT-MISMATCH), never overridden.
pub fn derived_source(words: Vec<u32>, planes: usize, g: &SourceGate, recorded_head: Option<u64>) -> Result<Ae32Source, ExecError> {
    let h = derived_head(planes, g)?;
    if let Some(r) = recorded_head {
        if r != h {
            return Err(ExecError::Shape(format!("SOURCE-VARIANT-MISMATCH: the native derives head {} from {:?}, the record shows {}", head_name(h), g, head_name(r))));
        }
    }
    Ok(Ae32Source { words, reader_head: h, planes })
}
/// The curve table pointer on the route (TABLE-NULL-ON-ROUTE): the native state carries no table.
pub const CURVE_TABLE_ON_ROUTE: Option<&'static [f32]> = None;
/// The stack-word globals on the route (ROUTE-CONST = 0).
pub const STACK_WORD_ON_ROUTE: u32 = 0;
/// READER-COUNT-RANGE bound: the block bound of the original wraps above this pixel count.
pub const READER_COUNT_MAX: usize = (i32::MAX as usize) - 15;

/// One 'ae32' source as the host gave it, with what the run's record says about it. No Default.
#[derive(Clone, Debug)]
pub struct Ae32Source {
    /// host memory, raw u32 words, rows of `width` pixels, A,R,G,B per pixel (16 bytes)
    pub words: Vec<u32>,
    /// the reader head recorded by the head hooks for this source (Batch 43 rule 1)
    pub reader_head: u64,
    /// the plane count recorded for this source (Batch 42 rule 6: no plane-count default)
    pub planes: usize,
}

/// The 'ae32' route of a HostConvert: one entry per source index, and the stack word of the destination request (0 on the route).
#[derive(Clone, Debug)]
pub struct Ae32Route {
    pub sources: Vec<Ae32Source>,
    pub dest_stack_word: u32,
    /// INT10: the destination record (None: convert-to-dest is refused, DEST-RECORD-REQUIRED)
    pub dest: Option<Ae32Dest>,
}

fn rerr(head: u64, e: ReadErr) -> ExecError {
    ExecError::Shape(format!("reader {}: {:?} (refused by name, ORIGINAL-UNDEFINED or a short buffer)", head_name(head), e))
}

/// convert-from-source on the 'ae32' route: every row through the recorded reader body; the image has the recorded plane count.
pub fn ae32_to_image(s: &Ae32Source, w: usize, h: usize) -> Result<Image, ExecError> {
    if w > READER_COUNT_MAX {
        return Err(ExecError::Shape(format!("CONTRACT-LIMIT READER-COUNT-RANGE: a line of {} pixels is above 2^31 - 16", w)));
    }
    if s.words.len() != 4 * w * h {
        return Err(ExecError::Shape(format!("'ae32' host memory has {} words, {}x{}x4 needed", s.words.len(), w, h)));
    }
    let want = match s.reader_head {
        R4_HEAD => 4,
        R3_HEAD => 3,
        R1_HEAD => 1,
        VB4_HEAD => 4,
        VB3_HEAD => 3,
        VB1_HEAD => 1,
        hd => {
            let listed = if UNPORTED_HEADS.contains(&hd) { "a Batch 43 rule 1 head" } else { "an unlisted head" };
            return Err(ExecError::Shape(format!("READER-BODY-NOT-PORTED: head 0x{:x} ({}) executed; ported and floored by FMT3e first, it blocks END-TO-END", hd, listed)));
        }
    };
    if s.planes != want {
        return Err(ExecError::Shape(format!("reader head {} writes {} planes, the record gives {}", head_name(s.reader_head), want, s.planes)));
    }
    let vb = matches!(s.reader_head, VB4_HEAD | VB3_HEAD | VB1_HEAD);
    // INT21 step G3R (W476 (d) 4, W170): the routing mutant g3r_mut_r3b sends R3 to the variant-B line; never in a shipped build
    let vb = vb || (cfg!(feature = "g3r_mut_r3b") && s.reader_head == R3_HEAD);
    let mut out: Vec<Vec<u32>> = vec![vec![0u32; w * h]; want];
    for y in 0..h {
        let src = &s.words[4 * w * y..4 * w * (y + 1)];
        let (head, rest) = out.split_at_mut(1);
        let r = match want {
            4 => {
                let (b, cd) = rest.split_at_mut(1);
                let (c, d) = cd.split_at_mut(1);
                let mut p: [&mut [u32]; 4] = [&mut head[0][w * y..w * (y + 1)], &mut b[0][w * y..w * (y + 1)], &mut c[0][w * y..w * (y + 1)], &mut d[0][w * y..w * (y + 1)]];
                if vb {
                    read4b(src, &mut p, 0, w as i32, false).map_err(|e| ExecError::Shape(format!("variant-B reader {}: {:?}", head_name(s.reader_head), e)))?;
                    Ok(())
                } else {
                    read4(src, &mut p, 0, w as i32, false)
                }
            }
            3 => {
                let (b, c) = rest.split_at_mut(1);
                let mut p: [&mut [u32]; 3] = [&mut head[0][w * y..w * (y + 1)], &mut b[0][w * y..w * (y + 1)], &mut c[0][w * y..w * (y + 1)]];
                if vb {
                    read3b(src, &mut p, 0, w as i32, false).map_err(|e| ExecError::Shape(format!("variant-B reader {}: {:?}", head_name(s.reader_head), e)))?;
                    Ok(())
                } else {
                    read3(src, &mut p, 0, w as i32, false)
                }
            }
            _ => {
                let mut p: [&mut [u32]; 1] = [&mut head[0][w * y..w * (y + 1)]];
                if vb {
                    read1b(src, &mut p, 0, w as i32, false).map_err(|e| ExecError::Shape(format!("variant-B reader {}: {:?}", head_name(s.reader_head), e)))?;
                    Ok(())
                } else {
                    read1(src, &mut p, 0, w as i32, false)
                }
            }
        };
        r.map_err(|e| rerr(s.reader_head, e))?;
    }
    Image::from_planes(out.into_iter().map(|p| p.into_iter().map(f32::from_bits).collect()).collect())
}

/// INT10: what the run's record says about the destination convert (Batch 42 rule 6, Batch 43 rules 2, 5 and 12). No Default.
#[derive(Clone, Debug)]
pub struct Ae32Dest {
    /// the plane count at the destination head (recorded; must equal the root image's channels)
    pub planes: usize,
    /// the mode argument of the destination call (r8d / [rbp], the alpha mode [ctx]; recorded per convert call)
    pub mode: i32,
    /// HOST-DEST-PRIOR: the host output buffer before the render, A,R,G,B u32 words, 4 * width * height (what the bodies leave untouched)
    pub prior: Vec<u32>,
    /// the row copier for the pair (14, 14): Some(i) = word 0 of every destination row is copied from the 'ae32'
    /// source i's row (the record names which buffer the rdx row belongs to); None = no copier call recorded
    pub copier_source: Option<usize>,
}

/// convert-to-dest on the 'ae32' route (INT10, FMT3c's crate c14): "DEST-ROUTE (code 14, stack word 0: zero-arm bodies, tier A: 4 planes from
/// 3 planes, 2 planes from, 1 plane; word 0 by the row copier body where recorded)",
/// labelled "R-PARITY (FLOOR PENDING)" (FMT3c: 12 of 12 plane count x tier sets at R-PARITY, 2,458,560 floats, 0 differing; the four plain
/// bodies are not FLOOR-MET, 122 of 651 mutants unproven; the copier (14, 14) is FLOOR-MET). Cell 1, the full row of `w` pixels per call.
/// Tier A only (feature word 0xffffff on the route; tiers B and C HOST-GATED; c14 is tier-generic as FMT3c ported it).
/// DEST-WORDS: words 1 to 3 from the body; word 0 from the body in modes 1 and 3 (and always for 4 and 2 planes), from the row copier in
/// mode 2 when recorded, else the prior content (HOST-DEST-PRIOR). A copier at a mode other than 2 is refused (COPIER-ORDER-UNRECORDED: the
/// body and the copier both write word 0 and their order is not recorded). A mode the body rejects (the original logs and writes
/// nothing) is refused by name (DEST-MODE-LOGGED). A nonzero stack word is HOST-GATED.
pub fn ae32_dest(route: &Ae32Route, im: &Image, w: usize, h: usize) -> Result<Vec<u32>, ExecError> {
    if route.dest_stack_word != STACK_WORD_ON_ROUTE {
        return Err(ExecError::Shape(format!("HOST-GATED: destination stack word {} is nonzero (the code-13 curve writers and the stack-word bodies are off route; TABLE-NULL-ON-ROUTE)", route.dest_stack_word)));
    }
    let d = route.dest.as_ref().ok_or_else(|| ExecError::Shape("DEST-RECORD-REQUIRED: the 'ae32' destination needs the recorded plane count, mode, prior buffer and copier record (no default)".into()))?;
    if im.channels() != d.planes {
        return Err(ExecError::Shape(format!("the root image has {} planes, the record gives {} at the destination head", im.channels(), d.planes)));
    }
    if w > READER_COUNT_MAX || im.n != w * h || d.prior.len() != 4 * w * h {
        return Err(ExecError::Shape(format!("'ae32' destination: frame {}x{}, image {} samples, prior {} words", w, h, im.n, d.prior.len())));
    }
    if d.copier_source.is_some() && d.mode != 2 {
        return Err(ExecError::Shape(format!("COPIER-ORDER-UNRECORDED: a row copier record at mode {} (the body writes word 0 too; their order is not recorded)", d.mode)));
    }
    let bits: Vec<Vec<u32>> = im.planes.iter().map(|p| p.iter().map(|x| x.to_bits()).collect()).collect();
    // M3P routing mutant (build-time env M3P_MUT=1): mode 3 sent to the mode-2 body (slot 0 not written)
    let body_mode = if option_env!("M3P_MUT") == Some("1") && d.mode == 3 { 2 } else { d.mode };
    let mut out = d.prior.clone();
    for y in 0..h {
        let rows: Vec<&[u32]> = bits.iter().map(|p| &p[w * y..w * (y + 1)]).collect();
        let dst = &mut out[4 * w * y..4 * w * (y + 1)];
        // M3P2 mutant (build-time feature): a 1-plane mode 3 destination sent to the 3-plane body (zero planes stand in for the missing two)
        let zero_row = vec![0u32; w];
        let padded: Vec<&[u32]> = if cfg!(feature = "m3p2_threeplane_body_mut") && d.planes == 1 && d.mode == 3 { vec![rows[0], &zero_row[..], &zero_row[..]] } else { rows.clone() };
        let np_body = if cfg!(feature = "m3p2_threeplane_body_mut") && d.planes == 1 && d.mode == 3 { 3 } else { d.planes };
        c14::convert14(np_body, &padded, dst, w as i32, 1, body_mode).map_err(|e| match e {
            c14::CErr::BadMode(m) => ExecError::Shape(format!("DEST-MODE-LOGGED: mode {} on a {}-plane destination (the original logs and writes nothing)", m, d.planes)),
            other => ExecError::Shape(format!("code-14 destination body: {:?}", other)),
        })?;
        if let Some(si) = d.copier_source {
            let s = route.sources.get(si).ok_or_else(|| ExecError::MissingInput(format!("'ae32' copier source {}", si)))?;
            if s.words.len() != 4 * w * h {
                return Err(ExecError::Shape(format!("'ae32' copier source {} has {} words, {} needed", si, s.words.len(), 4 * w * h)));
            }
            c14::copy_word0(&s.words[4 * w * y..4 * w * (y + 1)], dst, w as i32).map_err(|e| ExecError::Shape(format!("row copier (14, 14): {:?}", e)))?;
        }
    }
    Ok(out)
}
