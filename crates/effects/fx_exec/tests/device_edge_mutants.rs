//! Single-edit mutants of the edge_detect gradient kernel source: each is its own OpenCL program build and must be refused by the
//! `edge_detect` self-check (a mismatch, or NaN payloads that differ). Run with --test-threads=1.

use fx_device::selfcheck::NanPayloads;
use fx_device::{Backend as Handle, DeviceKind, Session};
use fx_exec::device_ops;
use std::sync::Arc;

/// (name, text, replacement)
const MUTANTS: &[(&str, &str, &str)] = &[
    ("fma operand roles swapped", "r = ed_fma213(gx, gx, gysq);", "r = ed_fma213(gysq, gx, gx);"),
    ("interior difference operands swapped", "float gx = nanx_sub(b[cur + x + 1], b[cur + x - 1]);", "float gx = nanx_sub(b[cur + x - 1], b[cur + x + 1]);"),
    ("first column scale K replaced by C", "float gx0 = nanx_mul(k, nanx_sub(b[cur + 1], b[cur]));", "float gx0 = nanx_mul(c, nanx_sub(b[cur + 1], b[cur]));"),
    ("last column scale K replaced by 2C", "float g = nanx_mul(k, nanx_sub(b[cur + wd - 1], b[cur + wd - 2]));", "float g = nanx_mul(c2, nanx_sub(b[cur + wd - 1], b[cur + wd - 2]));"),
    ("row factors C and 2C swapped", "? c2 : c;", "? c : c2;"),
    ("first row only gets the doubled factor", "float v = (y == 0 || y + 1 == (size_t)h) ? c2 : c;", "float v = (y == 0) ? c2 : c;"),
    ("last row only gets the doubled factor", "float v = (y == 0 || y + 1 == (size_t)h) ? c2 : c;", "float v = (y + 1 == (size_t)h) ? c2 : c;"),
    ("contracted multiply-add replaced by two roundings", "float ed_fma213(float d, float s2, float s3) {\n    float r = fma(d, s2, s3);", "float ed_fma213(float d, float s2, float s3) {\n    float r = d * s2 + s3;"),
    ("interior column edge index", "nanx_sub(b[cur + x + 1], b[cur + x - 1])", "nanx_sub(b[cur + x + 1], b[cur + x])"),
    ("first row neighbour index", "size_t up = (y == 0) ? cur : cur - wd;", "size_t up = (y == 0) ? cur + wd : cur - wd;"),
    ("square root replaced", "float ed_sqrt(float x) {\n    float r = sqrt(x);", "float ed_sqrt(float x) {\n    float r = x * 0x1p-1f;"),
    ("last column vertical factor C instead of the row factor", "float gyl = nanx_mul(nanx_sub(b[up + wd - 1], b[down + wd - 1]), v);", "float gyl = nanx_mul(nanx_sub(b[up + wd - 1], b[down + wd - 1]), c);"),
    ("fma NaN priority addend first", "    if (d != d) {\n        return as_float(as_uint(d) | 0x400000u);\n    }\n    if (s2 != s2) {", "    if (s3 != s3) {\n        return as_float(as_uint(s3) | 0x400000u);\n    }\n    if (s2 != s2) {"),
    ("last column difference index", "nanx_sub(b[cur + wd - 1], b[cur + wd - 2])", "nanx_sub(b[cur + wd - 2], b[cur + wd - 2])"),
];

#[test]
fn edge_detect_source_mutants_are_refused() {
    let ctx = match fx_device::select(DeviceKind::Gpu) {
        Handle::Device(d) => d,
        other => {
            if std::env::var("SAPPHIRE_REQUIRE_DEVICE").as_deref() == Ok("1") {
                panic!("device required but unavailable: {}", other.describe());
            }
            println!("NOT RUN: no usable OpenCL device ({})", other.describe());
            return;
        }
    };
    let op = device_ops::DEVICE_OPS.iter().find(|o| o.class == "edge_detect").unwrap();
    let base = fx_device::kernels::device_program();
    let base_digest = fx_device::progcache::fnv(base.as_bytes());
    let mut prev = base_digest;
    let mut survivors = Vec::new();
    for (name, from, to) in MUTANTS {
        assert_eq!(base.matches(from).count(), 1, "mutant `{name}`: the edited text must occur once");
        let src = base.replacen(from, to, 1);
        let digest = fx_device::progcache::fnv(src.as_bytes());
        assert!(digest != base_digest && digest != prev, "mutant `{name}` equals the base or the previous mutant");
        prev = digest;
        let (dev, info) = fx_device::probe::open(ctx.info.index).unwrap();
        let session = Arc::new(Session::open_source(&dev, &info, &src, fx_device::kernels::BUILD_OPTIONS).unwrap());
        let (killed, why) = match device_ops::selfcheck(&session, op) {
            Err(e) => (true, format!("mismatch ({})", e.chars().take(120).collect::<String>())),
            Ok(t) if t.nan_payloads != NanPayloads::Same => (true, "NaN payloads differ".to_string()),
            Ok(_) => (false, "equal".to_string()),
        };
        println!("MUTANT {name} program {digest:016x}: {} | {why}", if killed { "KILLED" } else { "SURVIVED" });
        if !killed {
            survivors.push(*name);
        }
    }
    println!("MUTANTS: {} of {} killed", MUTANTS.len() - survivors.len(), MUTANTS.len());
    assert!(survivors.is_empty(), "surviving mutants: {survivors:?}");
}
