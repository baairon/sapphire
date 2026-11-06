//! VPS-only check: the computed tables against the copies of the plugin's tables.
//! The copies live under `VEC_DIR/effects/native_tables/` (`stab_file.hex`, `dtab_file.hex`); when absent the test prints NOT RUN.
use libm_sincos::tables;

fn read(name: &str) -> Option<String> {
    std::fs::read_to_string(effects_vecdir::path(&format!("native_tables/{}", name))).ok()
}

fn rows(txt: &str) -> Vec<(usize, Vec<u64>)> {
    txt.lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            let w: Vec<&str> = l.split_whitespace().collect();
            (w[0].parse().unwrap(), w[1..].iter().map(|h| u64::from_str_radix(h, 16).unwrap()).collect())
        })
        .collect()
}

#[test]
fn stab_equals_copy() {
    let Some(txt) = read("stab_file.hex") else {
        println!("NOT RUN stab_equals_copy: VEC_DIR/effects/native_tables/stab_file.hex is absent");
        return;
    };
    let r = rows(&txt);
    assert_eq!(r.len(), 64);
    for (k, w) in r {
        assert_eq!(&tables().s[k][..], &w[..], "stab entry {}", k);
    }
}

#[test]
fn dtab_equals_copy() {
    let Some(txt) = read("dtab_file.hex") else {
        println!("NOT RUN dtab_equals_copy: VEC_DIR/effects/native_tables/dtab_file.hex is absent");
        return;
    };
    let r = rows(&txt);
    assert_eq!(r.len(), 64);
    for (k, w) in r {
        assert_eq!(&tables().d[k][..], &w[..], "dtab entry {}", k);
    }
}
