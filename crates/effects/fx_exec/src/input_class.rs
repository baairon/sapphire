//! INT20 step G (RDR8 PATCH 1 to 4 with W450 (b)'s amendments): the per-input class derived natively from the rule and the
//! caller's class-1 test, transcribed from RDR8's CLASS.md and input_class_model.py (listing neutral_1.asm text).
//! Label: "INPUT-CLASS-GUARD (per-input class derived natively from the rule; claim routes refused unless every bound input
//! derives the class the kernels implement; RDR7, RDR8, W450 (b))".
//!
//! Cells: [owner], [owner], [prog] come from the route's derivation; the ctx row cells (
//! [owner+2i]) from the 'ae32' host model; [prog], [prog] and the byte at * from the style's input-class
//! record (W450 (b) 1). A record cell is read only when the derivation reaches it; a reached cell without a record value is refused by
//! name (INPUT-CLASS-CELL-UNRECORDED). No cell defaults.

/// The class the ported kernels implement (4 lanes).
pub const CLASS_IMPLEMENTED: u8 = 4;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ClassErr {
    /// a record cell the derivation reaches has no record value
    Unrecorded(&'static str),
    /// the name reaches an arm of that is not transcribed (CLASS.md step 4, after Matte)
    ArmNotPorted(String),
}

/// The cells of one derivation. `None` = not in the style's record.
#[derive(Clone, Debug)]
pub struct ClassCells {
    pub ae32_ready_c: u32,
    pub ae32_ready_d: u32,
    pub prog_mode_word: u32,
    pub prog_class_byte: Option<u8>,
    pub prog_flag_byte: Option<u8>,
    pub flag_2: Option<u8>,
}

/// ctx row cells of input i: dword [owner row i], byte [owner row i], signed word [owner+2i]
#[derive(Clone, Copy, Debug)]
pub struct RowCells {
    pub fmt_cell_a: u32,
    pub b_cell_o: u8,
    pub word_cell_a: i16,
}

const ZERO_PREFIX: [&str; 2] = ["Source", "Back"];
const ZERO_EXACT: [&str; 11] = ["Front", "Foreground", "From", "To", "Middle", "Dummy", "EdgeGen", "ZBuffer", "Z_Buffer", "Lens", "Track"];
const TAIL_ZERO_IF_F_CELL_A: [&str; 3] = ["Matte", "Occlusion", "Mask"];
const CLASS1_PREFIX: [&str; 6] = ["Matte", "Mask", "Matte from", "Occlusion", "ToObject", "Kernel"];

fn need(v: Option<u8>, cell: &'static str) -> Result<u8, ClassErr> {
    v.ok_or(ClassErr::Unrecorded(cell))
}

/// 's return value (0 or 1) for input `i` named `name`.
pub fn derive_ret(i: usize, name: &str, c: &ClassCells, r: &RowCells) -> Result<u8, ClassErr> {
    // step 1 
    let edi: u8 = if (r.fmt_cell_a & 0xffff_fffd) == 0 || (r.b_cell_o & 1) != 0 { 1 } else { 0 };
    // step 2 
    let ecx = if c.ae32_ready_d != 0 { c.ae32_ready_d } else { c.prog_mode_word };
    // step 3 : bit i of the sign-extended byte [prog]; read only when ecx != 1
    let ebx: u8 = if ecx == 1 {
        edi
    } else {
        let b = need(c.prog_class_byte, "[prog]")? as i8 as i32;
        if i < 32 && (b >> i) & 1 == 1 {
            edi
        } else {
            0
        }
    };
    // step 4: [owner] == 0 takes the name arms
    if c.ae32_ready_c == 0 {
        if ZERO_PREFIX.iter().any(|p| name.starts_with(p)) || ZERO_EXACT.contains(&name) {
            return Ok(0);
        }
        if name == "Kernel" {
            // to the tail with ebx unchanged 
        } else if name == "Matte" || name == "Occlusion" {
            if need(c.prog_flag_byte, "[prog]")? & 0x40 != 0 {
                return Ok(0);
            }
            if name == "Occlusion" {
                return Err(ClassErr::ArmNotPorted("Occlusion: arms after not transcribed".into()));
            }
        } else {
            return Err(ClassErr::ArmNotPorted(format!("{}: arms after not transcribed", name)));
        }
    }
    // step 5, tail 
    if ebx == 0 {
        return Ok(0);
    }
    if TAIL_ZERO_IF_F_CELL_A.contains(&name) && need(c.prog_flag_byte, "[prog]")? & 0x40 != 0 {
        return Ok(0);
    }
    Ok(ebx)
}

/// The class byte the caller stores (then): 4 if the return is nonzero, else 3; 1 by the name test.
pub fn derive_class(i: usize, name: &str, c: &ClassCells, r: &RowCells) -> Result<u8, ClassErr> {
    let ret = derive_ret(i, name, c, r)?;
    let mut cls = if ret != 0 { 4 } else { 3 };
    if CLASS1_PREFIX.iter().any(|p| name.starts_with(p)) && r.word_cell_a > 0 && need(c.flag_2, "*")? != 0 {
        cls = 1;
    }
    Ok(cls)
}

/// One bound input's derived class, or the refusal text.
pub fn guard_one(style: &str, i: usize, name: &str, c: &ClassCells, r: &RowCells) -> Result<u8, String> {
    match derive_class(i, name, c, r) {
        Ok(k) => Ok(k),
        Err(ClassErr::Unrecorded(cell)) => Err(format!("INPUT-CLASS-CELL-UNRECORDED ({}: {})", style, cell)),
        Err(ClassErr::ArmNotPorted(e)) => Err(format!("INPUT-CLASS-NOT-PORTED ({} input {}: class derivation arm not ported: {})", style, name, e)),
    }
}

/// INPUT-CLASS-GUARD at step G: every bound input must derive CLASS_IMPLEMENTED. Returns the derived classes (position, name, class).
pub fn input_class_guard(style: &str, bound: &[(usize, &str)], c: &ClassCells, r: &RowCells) -> Result<Vec<(usize, String, u8)>, String> {
    let mut out = Vec::new();
    for &(i, name) in bound {
        let k = guard_one(style, i, name, c, r)?;
        if k != CLASS_IMPLEMENTED {
            return Err(format!("INPUT-CLASS-NOT-PORTED ({} input {}: derived class {}; consumers of a non-uniform class not ported)", style, name, k));
        }
        out.push((i, name.to_string(), k));
    }
    Ok(out)
}

/// Parse a record cell: a hex byte, or "-" for not in the record.
pub fn parse_cell(s: &str) -> Result<Option<u8>, String> {
    let t = s.trim();
    if t == "-" {
        return Ok(None);
    }
    u8::from_str_radix(t.trim_start_matches("0x"), 16).map(Some).map_err(|_| format!("not a hex byte or '-': {}", s))
}
