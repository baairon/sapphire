//! INT15 (UNT3 PATCH 2 with UNT5 PATCH 1 to 3): the parameter checkout rule, ported as read.
//!
//! Parse time, per parameter record (program text flags; listing parser ..0x1062xx):
//!   show_with_mask or mask_only: ba = position of the input named "Mask", else "Matte", else "Occlusion" (lookups);
//!   bg_only: ba = position of the input "Background", else "Back";
//!   show_with_mocha: bc |= 1;
//!   otherwise ba = -1, bc = 0. A ':name' token binds nothing (UNT5, MEASURED: plain Diffuse parses with ba = -1); the registered program
//!   of a style is its _Autogen text, in which the colon names are expanded (UNT5).
//! Run time (fetch gate family): the host checkout runs only when the record stamp is stale and REFRESH holds:
//!   ba < 0: REFRESH = (owner != NULL); ba >= 0: REFRESH = bit ba of [owner] (the input presence mask) or (bc & 1 and [owner] & 0x10).
//! Label: "CHECKOUT-RULE-READ (UNT3, UNT4, UNT5)". Report-only on this tree: no route consumes it yet (INT15-A2).

use crate::build::Genotype;

const MASK_NAMES: [&str; 3] = ["Mask", "Matte", "Occlusion"];
const BG_NAMES: [&str; 2] = ["Background", "Back"];

/// The parse-time bytes of one parameter record: [rec] (ba, -1 when no lookup succeeds) and bit 0 of [rec] (bc).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CheckoutBytes {
    pub ba: i16,
    pub bc: u8,
}

/// Arms of the parser that the rule uses; all true on the claim. A false arm exists only for the mutant test (UNT5: a model without the
/// show_with_mask arm must fail the Diffuse rows).
#[derive(Debug, Clone, Copy)]
pub struct Arms {
    pub show_with_mask: bool,
    pub mask_only: bool,
    pub bg_only: bool,
    pub show_with_mocha: bool,
}

impl Arms {
    pub const READ: Arms = Arms { show_with_mask: true, mask_only: true, bg_only: true, show_with_mocha: true };
}

fn find(inputs: &[&str], names: &[&str]) -> i16 {
    names.iter().find_map(|n| inputs.iter().position(|i| i == n)).map(|p| p as i16).unwrap_or(-1)
}

/// ba and bc of one record from its raw flag tokens and the program's input names in declared order.
pub fn bind_bytes(flags: &[String], inputs: &[&str], arms: Arms) -> CheckoutBytes {
    // INT17 (AUD1 1e F3): the parser stores ba at each arm keyword in token order, so the later token's ba stands
    let mut ba = -1;
    let mut bc = 0;
    for f in flags {
        match f.as_str() {
            "show_with_mask" if arms.show_with_mask => ba = find(inputs, &MASK_NAMES),
            "mask_only" if arms.mask_only => ba = find(inputs, &MASK_NAMES),
            "bg_only" if arms.bg_only => ba = find(inputs, &BG_NAMES),
            "show_with_mocha" if arms.show_with_mocha => bc |= 1,
            _ => {}
        }
    }
    CheckoutBytes { ba, bc }
}

/// REFRESH of the fetch gate for one record (the stamp test is taken as stale, as in every recorded compile).
pub fn refresh(b: CheckoutBytes, owner_fetch_byte: u8, owner_stamp_byte: u8, owner_nonnull: bool) -> bool {
    if b.ba < 0 {
        return owner_nonnull;
    }
    if b.ba < 8 && (owner_fetch_byte >> b.ba) & 1 == 1 {
        return true;
    }
    (b.bc & 1) == 1 && (owner_stamp_byte & 0x10) != 0
}

/// INT17 (AUD1 1e F1 and F4): refusals where the crate and the listing would part.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckoutError {
    /// ba >= 32: `bt eax,edx` takes the index modulo 32 in the listing ("CHECKOUT-BA-OUT-OF-RANGE")
    BaOutOfRange(i16),
    /// owner NULL with ba >= 0: the listing reads [owner] and faults ("CHECKOUT-OWNER-NULL")
    OwnerNull,
}

/// The fetch gate in the listing's form: bt on the zero-extended byte [owner] (bit ba clear for 8 <= ba < 32), refusals by name.
pub fn try_refresh(b: CheckoutBytes, owner_fetch_byte: u8, owner_stamp_byte: u8, owner_nonnull: bool) -> Result<bool, CheckoutError> {
    if b.ba < 0 {
        return Ok(owner_nonnull);
    }
    if b.ba >= 32 {
        return Err(CheckoutError::BaOutOfRange(b.ba));
    }
    if !owner_nonnull {
        return Err(CheckoutError::OwnerNull);
    }
    if ((owner_fetch_byte as u32) >> (b.ba as u32)) & 1 == 1 {
        return Ok(true);
    }
    Ok((b.bc & 1) == 1 && (owner_stamp_byte & 0x10) != 0)
}

/// Per parameter of the genotype, in declared order: (name, bytes, checked out) at the given owner cells.
pub fn checkout_table(g: &Genotype, owner_fetch_byte: u8, owner_stamp_byte: u8, arms: Arms) -> Vec<(String, CheckoutBytes, bool)> {
    let mut ins: Vec<(usize, &str)> = g.inputs.iter().map(|i| (i.pos, i.name.as_str())).collect();
    ins.sort();
    let names: Vec<&str> = ins.iter().map(|x| x.1).collect();
    g.params
        .iter()
        .map(|p| {
            let b = bind_bytes(&p.flags, &names, arms);
            (p.name.clone(), b, refresh(b, owner_fetch_byte, owner_stamp_byte, true))
        })
        .collect()
}

/// The names of the parameters that are NOT checked out at the given owner cells (the host value never reaches them; they keep the cached lanes).
pub fn not_checked_out(g: &Genotype, owner_fetch_byte: u8, owner_stamp_byte: u8) -> Vec<String> {
    checkout_table(g, owner_fetch_byte, owner_stamp_byte, Arms::READ).into_iter().filter(|x| !x.2).map(|x| x.0).collect()
}
