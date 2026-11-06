//! the tail census. Writes DS_TAIL_CENSUS_RAW.tsv (path in env DS3_CENSUS_OUT) for every program at blur_res 1 and 2, do_postdiv true and false.
mod t_ds3_common;
use t_ds3_common::*;
use fx_exec::exec::compile_for;
#[test]
fn tail_census() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        return;
    }
    let t = std::fs::read_to_string(pt()).unwrap();
    let mut names: Vec<String> = t.lines().filter(|l| l.starts_with("def_effect\t")).filter_map(|l| l.split('\t').nth(1)).filter(|s| !s.is_empty()).map(|s| s.to_string()).collect();
    names.sort();
    names.dedup();
    let mut rows = vec!["program\tblur_res\tdo_postdiv\tds_factor\ttail_len\ttail_kinds_prims".to_string()];
    for n in names {
        let c = fx_eval::Corpus::from_programs_tsv(&t);
        let Some(ei) = c.find_effect(&n, None) else { continue };
        let Ok(g) = fx_eval::build_effect(&c, ei) else { continue };
        for br in [1.0f32, 2.0] {
            for pd in [true, false] {
                let mut b = binding(br, pd);
                b.overrides.insert("blur_res".into(), vec![br]);
                let r = std::panic::catch_unwind(|| compile_for(&g, &b).ok().map(|cp| {
                    let tl = fx_eval::cpuplan::ds_tail(&cp);
                    (fx_eval::cpuplan::ds_factor(&cp), tl.iter().map(|&i| format!("{}:{}", cp.nodes[i].kind, cp.nodes[i].prim)).collect::<Vec<_>>())
                }));
                if let Ok(Some((f, tl))) = r {
                    if f > 1 { rows.push(format!("{}\t{}\t{}\t{}\t{}\t{}", n, br, pd, f, tl.len(), tl.join(","))); }
                }
            }
        }
    }
    if let Ok(p) = std::env::var("DS3_CENSUS_OUT") { std::fs::write(p, rows.join("\n") + "\n").unwrap(); }
}
