//! `unsafe` code lives in `src/cl.rs` only; every other source file of the crate is scanned for the keyword.

use std::path::Path;

fn scan(dir: &Path, hits: &mut Vec<String>) {
    for e in std::fs::read_dir(dir).unwrap() {
        let p = e.unwrap().path();
        if p.is_dir() {
            scan(&p, hits);
        } else if p.extension().is_some_and(|x| x == "rs") && p.file_name().unwrap() != "cl.rs" && p.file_name().unwrap() != "no_unsafe.rs" {
            let text = std::fs::read_to_string(&p).unwrap();
            let kw = ["un", "safe"].concat();
            for (i, line) in text.lines().enumerate() {
                let code = line.split("//").next().unwrap_or("");
                if code.split(|c: char| !c.is_alphanumeric() && c != '_').any(|w| w == kw) {
                    hits.push(format!("{}:{}", p.display(), i + 1));
                }
            }
        }
    }
}

#[test]
fn only_cl_rs_contains_unsafe_code() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut hits = Vec::new();
    scan(&root.join("src"), &mut hits);
    scan(&root.join("tests"), &mut hits);
    assert!(hits.is_empty(), "unsafe outside cl.rs: {hits:?}");
}
