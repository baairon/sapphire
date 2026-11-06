//! Step the D_64x32 claim line of Drop_Shadow on the 'ae32' route compiles with a 4-plane root and the
//! v3_to_v4_if4 node folded at n 4 to [0 0 0 1]. The line is read from dsc1()'s stored PRED.tsv (its 'cmd' column) so that this
//! file carries none of the recorded words; only the file paths are redirected. Kills the mutant 'Output at the minimal-context
//! default' (the node folds at 3 lanes and the root has 3 planes).
use std::process::Command;

fn pt() -> &'static str { effects_vecdir::path_str("record_extract/programs/programs.tsv") }
fn dsc1() -> &'static str { effects_vecdir::path_str("wave_s9/dsc1_dropshadow_counted") }

fn claim_args(out: &str) -> Vec<String> {
    let t = std::fs::read_to_string(format!("{}/trial/PRED.tsv", dsc1())).expect("PRED.tsv");
    let mut lines = t.lines();
    let head: Vec<&str> = lines.next().unwrap().split('\t').collect();
    let ci = head.iter().position(|h| *h == "cmd").expect("cmd column");
    let row = lines.find(|l| l.starts_with("D_64x32\t")).expect("D_64x32 row");
    let cmd = row.split('\t').nth(ci).unwrap();
    // the PRED cmd starts at the program name; fx_run takes the programs table first
    let mut a: Vec<String> = vec![pt().to_string()];
    a.extend(cmd.split(' ').map(|s| s.to_string()));
    for i in 0..a.len() {
        if i > 0 && (a[i - 1] == "--out" || a[i - 1] == "--ae32-out") {
            a[i] = if a[i - 1] == "--out" { out.to_string() } else { format!("{}.ae32.u32", out) };
        } else if let Some(r) = a[i].strip_prefix("trial/") {
            a[i] = format!("{}/trial/{}", dsc1(), r);
        } else if let Some((k, r)) = a[i].split_once("=trial/") {
            a[i] = format!("{}={}/trial/{}", k, dsc1(), r);
        }
    }
    a
}

#[test]
fn d64_claim_line_has_a_four_plane_root_and_the_0001_node() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv", "wave_s9/dsc1_dropshadow_counted/trial/PRED.tsv", "wave_s9/dsc1_dropshadow_counted"]) {
        return;
    }
    let dir = std::env::temp_dir().join(format!("t_dx2_claim_d64_{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let out = dir.join("D").to_str().unwrap().to_string();
    let o = Command::new(env!("CARGO_BIN_EXE_fx_run")).args(claim_args(&out)).output().expect("fx_run");
    let err = String::from_utf8_lossy(&o.stderr).to_string();
    assert_eq!(o.status.code(), Some(0), "{}", err);
    let meta = std::fs::read_to_string(format!("{}.dest.meta.txt", out)).expect("meta");
    assert!(meta.contains("channels 4\n"), "{}", meta);
    let f: Vec<&str> = meta.lines().filter(|l| l.starts_with("diagnostic V3TOV4IF4-FOLD\t")).collect();
    assert_eq!(f.len(), 1, "{}", meta);
    assert!(f[0].contains("\tn 4\t"), "{}", f[0]);
    assert!(f[0].contains("\tresult 00000000,00000000,00000000,3f800000\t"), "{}", f[0]);
    let _ = std::fs::remove_dir_all(&dir);
}
