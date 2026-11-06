//! INT27 step SI (SIP1 outcome A): the 'ae32' route of a compiled plan that has no Source input and is a single folded
//! constant node (out_ch 3). The original's plans at G0, GC, GD, GN with Source at 3 channels are command 0 = allocate 3 / constant / convert-to-dest,
//! with no Source and no convert-from-source (SIP1, MEASURED in the emulator). The native renders the constant lanes through the destination
//! convert, the host Source world given and its planes unread, ONLY when the derived Source class is 3 with 3 planes. Every other plan without a
//! required input keeps the refusal by name; a Source-independent plan at 4 planes refuses SOURCE-INDEPENDENT-4CH-UNPORTED.
use fx_eval::CompiledProgram;

/// The refusal kept for every other plan that lacks a required input (the pre-SIP1 text, unchanged).
pub fn not_an_input(nm: &str) -> String {
    format!("'ae32' source {} is not an input of the compiled plan", nm)
}

/// The constant lanes (f32 bits, lane 3 included) when the plan is one folded constant node with 3 output channels and no input node.
pub fn single_const3(cp: &CompiledProgram) -> Option<[u32; 4]> {
    if cp.nodes.len() != 1 || cp.nodes.iter().any(|n| n.kind == "input") {
        return None;
    }
    let n = &cp.nodes[0];
    if n.kind == "const" && n.folded && n.elnum == 3 {
        Some(n.lanes)
    } else {
        None
    }
}

/// The route decision. `names`: the 'ae32' source names with their recorded plane counts; `source_elnum`: the element count the class-3
/// derivation set for Source before compile (Some(3) for class 3). Ok(false): every source is an input (the ordinary route); Ok(true): the
/// Source-independent constant route; Err: the refusal by name.
pub fn route_check(cp: &CompiledProgram, names: &[(String, usize)], source_elnum: Option<i32>, style: &str, binding: &str) -> Result<bool, String> {
    let missing: Vec<&(String, usize)> = names.iter().filter(|(nm, _)| !cp.nodes.iter().any(|nd| nd.kind == "input" && nd.name == *nm)).collect();
    if missing.is_empty() {
        return Ok(false);
    }
    if let Some((nm, _)) = missing.iter().find(|(nm, _)| nm != "Source") {
        return Err(not_an_input(nm));
    }
    let planes = missing[0].1;
    if single_const3(cp).is_none() {
        return Err(not_an_input("Source"));
    }
    if planes == 4 {
        return Err(format!("SOURCE-INDEPENDENT-4CH-UNPORTED ({} {}: the native compiled a Source-independent plan at 4 planes; the original's 4-channel plan reads Source (SIP1); W552)", style, binding));
    }
    if planes == 3 && source_elnum == Some(3) {
        return Ok(true);
    }
    Err(not_an_input("Source"))
}

/// The meta label of the constant route.
pub fn label(style: &str, binding: &str, lanes: &[u32; 4]) -> String {
    format!(
        "label SOURCE-INDEPENDENT-CONST-ROUTE ({} {}: the compiled plan has no Source input and is one folded constant node v3_to_v4_if4 (out_ch 3, lanes {:08x},{:08x},{:08x}); rendered through the destination convert (3-plane zero-arm body at mode 2 by FMT3c, or the mode-3 body), the host Source world given and its planes unread; Source class 3 with 3 planes; booking needs SOURCE-INDEPENDENT-PLAN-RECORDED per row; SIP1, W552, W561)",
        style, binding, lanes[0], lanes[1], lanes[2]
    )
}
