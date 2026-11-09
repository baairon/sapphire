/// Number of frame cells the cache initialiser stores per 0x280 byte group.
pub const INIT_STORE_COUNT: i32 = 64;

/// Byte offset, from the loop pointer, of the `i`th frame cell the cache initialiser stores (`i` below `INIT_STORE_COUNT`):
/// four blocks of 16 cells at stride 0xa0; inside a block two half-words 4 bytes apart, each holding eight slots 0x14 apart.
pub fn init_store_offset(i: i32) -> i32 {
    (i / 16) * 0xa0 + ((i / 8) % 2) * 4 + (i % 8) * 0x14
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_cells_follow_the_block_layout() {
        assert_eq!(init_store_offset(0), 0);
        assert_eq!(init_store_offset(7), 0x8c);
        assert_eq!(init_store_offset(8), 0x4);
        assert_eq!(init_store_offset(15), 0x90);
        assert_eq!(init_store_offset(16), 0xa0);
        assert_eq!(init_store_offset(63), 0x270);
    }

    #[test]
    fn the_cells_are_distinct_and_inside_one_group() {
        let mut seen = std::collections::BTreeSet::new();
        for i in 0..INIT_STORE_COUNT {
            let o = init_store_offset(i);
            assert!((0..0x280).contains(&o), "{}", o);
            assert!(seen.insert(o), "offset {:#x} repeats", o);
        }
        assert_eq!(seen.len(), INIT_STORE_COUNT as usize);
    }
}
