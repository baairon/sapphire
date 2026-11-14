//! The level checks at the head of each pyramid level and the unordered compare predicates of the body.
use crate::iface::{LevelCheck, LevelRecord};

/// Ordered greater-than (a compare of the form `vcomiss` then `ja`): false for an unordered pair. Sites in the body: the range test
/// of plane A (0x47b311), the range test of plane B (0x47b3b9); the scan compares use `spanloops::scan_step`; the eps gate of the
/// glue is `gluefn::eps_gate`.
pub fn gt_ordered(a: f32, b: f32) -> bool {
    a > b
}

/// The head of a level (listing 0x47af8b..0x47b016): the flag word first, then the four dimensions against 2 (signed compares).
/// `Proceed` needs previous width and height and current width and height all at least 2.
pub fn level_checks(prev: &LevelRecord, cur: &LevelRecord, flag_route: bool) -> LevelCheck {
    if flag_route {
        return LevelCheck::FlagRoute;
    }
    if prev.width >= 2 && prev.height >= 2 && cur.width >= 2 && cur.height >= 2 {
        LevelCheck::Proceed
    } else {
        LevelCheck::SmallSide
    }
}

/// True when any level of the table has a side below 2 (signed compare). The level build keeps every stored side at max(2, levels
/// parameter) or more, so a table with such a side never arises from the build: the body refuses it by name.
pub fn has_small_side(levels: &[LevelRecord]) -> bool {
    levels.iter().any(|l| l.width < 2 || l.height < 2)
}
