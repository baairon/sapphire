//! The `pin ADDR VALUE` case line: a pinned world word reads as its value, every other word is unchanged,
//! and a case without pin lines reads exactly as before.

use neutral_world::{parse, Env, Layout, World};

const HEAD: &str = "# world vectors v1\nworld pin_test entry=0x0 n=111 anchor=0x1000006f stub_end=0xf91 args=0 rets=1 seed=5 ptrn=6 heap=aligned fetch=byte\n";
const PINNED: u64 = 0x1014_03d8;
const VALUE: u64 = 0x1122_3344_5566_7788;

fn two_cases() -> String {
    format!(
        "{}case 0 ok seed=0x3eb2ecbc903907fc\npin 0x{:x} 0x{:x}\nabi ok\nr 0 eax 0x0\nend\ncase 1 ok seed=0x3eb2ecbc903907fc\nabi ok\nr 0 eax 0x0\nend\n",
        HEAD, PINNED, VALUE
    )
}

fn layout() -> Layout {
    Layout::new(111, 0xf91, 6).with_aligned_heap()
}

#[test]
fn pin_line_is_parsed_per_case() {
    let vf = parse(&two_cases()).unwrap();
    assert_eq!(vf.cases[0].pins, vec![(PINNED, VALUE)]);
    assert!(vf.cases[1].pins.is_empty());
}

#[test]
fn pinned_word_reads_as_its_value_and_neighbours_are_unchanged() {
    let vf = parse(&two_cases()).unwrap();
    let lay = layout();
    assert!(lay.in_world(PINNED));
    let seed = vf.cases[0].seed;
    let mut w = World::new(&vf.header, &vf.cases[0], None);
    assert_eq!(w.r64(PINNED), VALUE);
    assert_eq!(w.r8(PINNED + 1), 0x77);
    assert_eq!(w.r32(PINNED + 4), 0x1122_3344);
    assert_eq!(w.r64(PINNED + 8), lay.word(seed, PINNED + 8));
    assert_eq!(w.r64(PINNED - 8), lay.word(seed, PINNED - 8));
}

#[test]
fn a_case_without_pins_reads_the_seeded_word() {
    let vf = parse(&two_cases()).unwrap();
    let lay = layout();
    let mut w = World::new(&vf.header, &vf.cases[1], None);
    assert_eq!(w.r64(PINNED), lay.word(vf.cases[1].seed, PINNED));
    assert_ne!(lay.word(vf.cases[1].seed, PINNED), VALUE);
}

#[test]
fn a_stored_write_wins_over_the_pin() {
    let vf = parse(&two_cases()).unwrap();
    let mut w = World::new(&vf.header, &vf.cases[0], None);
    w.w64(PINNED, 7);
    assert_eq!(w.r64(PINNED), 7);
}

#[test]
fn pin_outside_a_case_is_an_error() {
    let text = format!("{}pin 0x{:x} 0x{:x}\n", HEAD, PINNED, VALUE);
    let err = parse(&text).unwrap_err();
    assert!(err.contains("pin outside case"), "{}", err);
}
