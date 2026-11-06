//! Step v2: a code pin for the own-count mutant. D_64x32's 'ae32' claim line (read from dsc1's trial PRED.tsv, as in
//! t_dx2_claim_d64) run at a constructed 1-plane destination: the real code folds node 65 at its bound image argument's count 1 and
//! refuses V3TOV4IF4-NARROW-UNMEASURED; the mutant dx2_owncount_mut folds at its own count 3 and fails later with a shape error.
//! A probe only: claimed rows are judged at their recorded destinations.
use std::process::Command;

fn pt() -> &'static str { effects_vecdir::path_str("record_extract/programs/programs.tsv") }
fn dsc1() -> &'static str { effects_vecdir::path_str("wave_s9/dsc1_dropshadow_counted") }

fn claim_args(out: &str, planes: &str) -> Vec<String> {
    let t = std::fs::read_to_string(format!("{}/trial/PRED.tsv", dsc1())).expect("PRED.tsv");
    let mut lines = t.lines();
    let head: Vec<&str> = lines.next().unwrap().split('\t').collect();
    let ci = head.iter().position(|h| *h == "cmd").expect("cmd column");
    let row = lines.find(|l| l.starts_with("D_64x32\t")).expect("D_64x32 row");
    let cmd = row.split('\t').nth(ci).unwrap();
    let mut a: Vec<String> = vec![pt().to_string()];
    a.extend(cmd.split(' ').map(|s| s.to_string()));
    for i in 0..a.len() {
        if i > 0 && (a[i - 1] == "--out" || a[i - 1] == "--ae32-out") {
            a[i] = if a[i - 1] == "--out" { out.to_string() } else { format!("{}.ae32.u32", out) };
        } else if i > 0 && a[i - 1] == "--ae32-dest" {
            let mode = a[i].split_once(',').map(|x| x.1.to_string()).unwrap_or_default();
            a[i] = format!("{},{}", planes, mode);
        } else if let Some(r) = a[i].strip_prefix("trial/") {
            a[i] = format!("{}/trial/{}", dsc1(), r);
        } else if let Some((k, r)) = a[i].split_once("=trial/") {
            a[i] = format!("{}={}/trial/{}", k, dsc1(), r);
        }
    }
    a
}

#[test]
fn d64_at_a_one_plane_destination_folds_node_65_at_the_bound_count() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv", "wave_s9/dsc1_dropshadow_counted/trial/PRED.tsv", "wave_s9/dsc1_dropshadow_counted"]) {
        return;
    }
    let dir = std::env::temp_dir().join(format!("t_dx2_owncount_dest_{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let out = dir.join("D1").to_str().unwrap().to_string();
    let o = Command::new(env!("CARGO_BIN_EXE_fx_run")).args(claim_args(&out, "1")).output().expect("fx_run");
    let err = String::from_utf8_lossy(&o.stderr).to_string();
    assert_eq!(o.status.code(), Some(2), "{}", err);
    assert!(err.contains("V3TOV4IF4-NARROW-UNMEASURED (Drop_Shadow: node 65: n 1,"), "{}", err);
}
