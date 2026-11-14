//! Named refusals of the optical flow body: decided from the level table and the host flag word before the first launch, so a run
//! that reaches one ends with exit status 2 and no output frame.
use crate::body::Refusal;

/// Exit status of a refused run.
pub const EXIT_STATUS: i32 = 2;

pub const SMALL_LEVEL: &str = "OPTICAL-FLOW-SMALL-LEVEL-REFUSED (optical flow: a pyramid level with a width or height below 2 is outside the served level table, so the frame is not rendered)";
pub const LEVEL_TABLE_FULL: &str = "OPTICAL-FLOW-LEVEL-TABLE-FULL-REFUSED (optical flow: the pyramid would hold more levels than the level table's capacity, so the frame is not rendered)";
pub const NULL_IMAGE: &str = "OPTICAL-FLOW-NULL-IMAGE-REFUSED (optical flow: the host answered no image for a pyramid buffer, so the frame is not rendered)";
pub const FLAG_ROUTE: &str = "OPTICAL-FLOW-FLAG-ROUTE-REFUSED (optical flow: the host image handle route is not rendered)";

/// The log line of a refusal: its name first, then the reason. FlagRoute has two sites: Cut::Helper and Alt20 with the flag word.
pub fn line(r: Refusal) -> &'static str {
    match r {
        Refusal::FlagRoute => FLAG_ROUTE,
        Refusal::SmallLevel => SMALL_LEVEL,
        Refusal::LevelTableFull => LEVEL_TABLE_FULL,
        Refusal::NullImage => NULL_IMAGE,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_refusal_line_names_itself() {
        assert!(line(Refusal::FlagRoute).starts_with("OPTICAL-FLOW-FLAG-ROUTE-REFUSED"));
        assert!(line(Refusal::SmallLevel).starts_with("OPTICAL-FLOW-SMALL-LEVEL-REFUSED"));
        assert!(line(Refusal::LevelTableFull).starts_with("OPTICAL-FLOW-LEVEL-TABLE-FULL-REFUSED"));
        assert!(line(Refusal::NullImage).starts_with("OPTICAL-FLOW-NULL-IMAGE-REFUSED"));
        assert_eq!(EXIT_STATUS, 2);
    }
}
