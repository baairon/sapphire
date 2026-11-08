//! The destination-route label of a class-3 mode-2 run states the pairing of the source head and the destination body
//! (each at its own floor); it carries no pending-floor wording.
use std::path::PathBuf;
use std::process::Command;

const W: usize = 16;
const H: usize = 8;

fn dir(tag: &str) -> PathBuf {
    let d = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!("dest_route_label_{}_{}", std::process::id(), tag));
    std::fs::create_dir_all(&d).unwrap();
    d
}

fn run_flicker(tag: &str) -> (Option<i32>, String, String) {
    let d = dir(tag);
    let mut src = Vec::new();
    for k in 0..4 * W * H {
        let x = ((k * 7 + 5) % 31) as f32 / 31.0;
        src.extend_from_slice(&x.to_le_bytes());
    }
    let sp = d.join("src.u32");
    std::fs::write(&sp, &src).unwrap();
    let pp = d.join("prior.u32");
    std::fs::write(&pp, vec![0xaau8; 16 * W * H]).unwrap();
    let os = d.join("o").to_str().unwrap().to_string();
    let geom = format!("-0.5,{},1,0,{},0,0", W as f32 - 0.5, (H as f32 - W as f32) / 2.0 - 0.5);
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_fx_run"));
    cmd.args(["Flicker", "--size", &format!("{}x{}", W, H), "--out", &os]);
    cmd.args(["--ae32-src", &format!("Source={}", sp.display()), "--ae32-prior", pp.to_str().unwrap(), "--ae32-out", &format!("{}.ae32.u32", os)]);
    cmd.args(["--geom", &geom, "--host-frame", "0", "--ctx-time", "0", "--src-planes", "3", "--owner-arm", "0,0,0", "--src-row58", "0", "--src-head", "0x7e91c0"]);
    cmd.args(["--aspect-triple", "0,3f800000,3f800000", "--ae32-dest", "3,2", "--ae32-copier", "0", "--dest-stack-word", "0", "--units-ratio", "1,1", "--bound-mask", "1"]);
    cmd.args(["--ctx-word2", "0", "--ctx-1ae", "4", "--ctx-has-data", "f8", "--ranges", "REAL", "--prerender-late", "0,0,0", "--mode-inputs", "1,2"]);
    cmd.args(["--class-cells", "02,80,01", "--class3-record", "0,1,3,01", "--host-value", "get_time=0x00000000", "--ns-default-unbound", "--do-postdiv", "0"]);
    let p = cmd.output().expect("fx_run");
    let meta = std::fs::read_to_string(format!("{}.dest.meta.txt", os)).unwrap_or_default();
    (p.status.code(), String::from_utf8_lossy(&p.stderr).to_string(), meta)
}

#[test]
fn dest_route_label_mode2_states_head_body_pairing() {
    let (rc, err, meta) = run_flicker("m2");
    let all = format!("{}\n{}", err, meta);
    assert_eq!(rc, Some(0), "{}", all);
    let line = all.lines().find(|l| l.contains("label DEST-ROUTE")).unwrap_or_else(|| panic!("no DEST-ROUTE label:\n{}", all));
    assert!(line.contains("mode 2"), "{}", line);
    assert!(line.contains("HEAD-BODY-PAIR-BY-E2E"), "{}", line);
    assert!(!line.contains("FLOOR-PENDING"), "{}", line);
}
