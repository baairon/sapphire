//! WPG1 slot-function T2 for perspective_alpha: enters the dispatcher 0x548e00 (feature word served through its cell) in the emulator. Adapted from PX2 t2_planes.rs.
//! T2 planes: Rust port against the emulated original (harness v2, vex engine, SDM not silicon), run folders t2/out_planes_{A,B}/p<ph>/<set>/
//! (real code of the body, of sincos 0xb76580, of the ROI helper 0x5b9b10, of 0x6670a0 and 0x673100; memset hooks for the two memset paths,
//! PX2-A6; table entries 16..63 and the vector clamp cell are the labelled stand-ins PX2-A2 and PX2-A1).
//! Written samples compare as NaN class (silicon NaN rule: payload and sign of a NaN result are not compared); every other sample of the
//! plane (not written by either side) must be bit equal to the harness fill pattern.
use fx_common::oracle_io::Manifest;
use wpg1_glue::*;
const FW: u64 = 0x9d97ff;
use std::collections::BTreeMap;

fn same(a: u32, b: u32) -> bool {
    a == b || (f32::from_bits(a).is_nan() && f32::from_bits(b).is_nan())
}

fn ulp(a: u32, b: u32) -> u64 {
    let k = |u: u32| if u >> 31 == 1 { -((u & 0x7fff_ffff) as i64) } else { u as i64 };
    (k(a) - k(b)).unsigned_abs()
}

#[test]
fn planes_against_emulator() {
    if !effects_vecdir::have_all(&["crates/wpg1_glue/t2"]) {
        return;
    }
    let (mut sets, mut compared, mut floats, mut diff, mut maxulp, mut outc, mut outc_ok) = (0usize, 0usize, 0usize, 0usize, 0u64, 0usize, 0usize);
    let (mut written, mut interior, mut at_one, mut zeros, mut nans, mut fills, mut at_rmax_vec, mut fill_floats) = (0usize, 0usize, 0usize, 0usize, 0usize, 0usize, 0usize, 0usize);
    let mut sec = BTreeMap::<&str, usize>::new();
    let mut phs = BTreeMap::<usize, usize>::new();
    let mut first = String::new();
    let mut outputs: BTreeMap<(String, String), Vec<u32>> = BTreeMap::new();
    let mut roots: Vec<std::path::PathBuf> = Vec::new();
    let sfx = std::env::var("T2_SUFFIX").unwrap_or(String::new());
    for grp in [effects_vecdir::path(&format!("crates/wpg1_glue/t2/out_alpha_A{}", sfx)).to_string_lossy().into_owned(), effects_vecdir::path(&format!("crates/wpg1_glue/t2/out_alpha_B{}", sfx)).to_string_lossy().into_owned()] {
        let mut dirs: Vec<_> = std::fs::read_dir(&grp).expect("run the planes harness first").map(|e| e.unwrap().path()).collect();
        dirs.sort();
        roots.extend(dirs);
    }
    for root in roots {
        let mut names: Vec<_> = std::fs::read_dir(&root).unwrap().map(|e| e.unwrap().file_name().into_string().unwrap()).collect();
        names.sort();
        for nm in &names {
            let Ok(m) = Manifest::load(&root.join(nm)) else { continue };
            sets += 1;
            for c in &m.cases {
                let g = |k: &str| c.param_f64(k).unwrap();
                let par = AlphaParams { params_b: f32::from_bits(g("v88b") as u32), params_a: f32::from_bits(g("v6cb") as u32), params_c: f32::from_bits(g("va4b") as u32) };
                let info = AlphaInfo {
                    n: g("n") as i32,
                    roi_form: g("roi_form") as i32,
                    port_blk_a: g("f28") as i32,
                    port_blk_b: g("f2c") as i32,
                    port_blk_c: g("f44") as i32,
                    s8: g("s8") as i32,
                    port_info_b: g("s48") as i32,
                    port_info_a: f64::from_bits((g("s30lo") as u64) | ((g("s30hi") as u64) << 32)),
                };
                let rmax = f32::from_bits(g("rmaxb") as u32);
                let ph = g("ph") as usize;
                let phase = (ph + c.misalign_bytes) & 0x1f;
                let src = c.input("in").unwrap();
                let pattern = 0x7fc0ffeeu32;
                let mut got = vec![f32::from_bits(pattern); c.height * c.stride];
                let res = slot52_perspective_alpha_with(FW, rmax, &par, &info, g("flag") != 0.0, &src, &mut got[ph / 4..], phase);
                if !c.ok() {
                    outc += 1;
                    if matches!(res, Err(Slot52Error::Alpha(AlphaError::SincosOutOfContract))) {
                        outc_ok += 1;
                    } else {
                        println!("OUT OF CONTRACT mismatch {} {} status {} port {:?}", nm, c.name, c.status, res.map(|o| o.route));
                    }
                    continue;
                }
                let o = res.expect("glue refused a case the original ran");
                let want = c.output("out").unwrap();
                compared += 1;
                *phs.entry(phase).or_insert(0) += 1;
                // written range of the port (the fill count is bounded by the plane size by construction of the sets)
                let (lo, cnt) = match o.route {
                    AlphaRoute::Ramp => (ph / 4, if info.n > 0 { info.n as usize } else { 0 }),
                    AlphaRoute::Fill { count, flag: false } => (ph / 4, if count > 0 { count as usize } else { 0 }),
                    _ => (0, 0),
                };
                let mut cd = 0;
                for (i, (a, b)) in got.iter().zip(want.iter()).enumerate() {
                    floats += 1;
                    let inside = i >= lo && i < lo + cnt;
                    let ok = if inside { same(a.to_bits(), b.to_bits()) } else { a.to_bits() == b.to_bits() };
                    if !ok {
                        cd += 1;
                        maxulp = maxulp.max(ulp(a.to_bits(), b.to_bits()));
                        if first.is_empty() {
                            first = format!("set {} case {} sample {} got {:08x} want {:08x}", nm, c.name, i, a.to_bits(), b.to_bits());
                        }
                    }
                }
                diff += cd;
                written += cnt;
                match o.route {
                    AlphaRoute::Ramp => {
                        let pl = o.plan;
                        *sec.entry(match pl {
                            None => "n<=0",
                            Some(p) if p.vend == 0 => "all scalar",
                            Some(p) if p.peel > 0 && p.vend < info.n as usize => "peel+vector+tail",
                            Some(p) if p.peel > 0 => "peel+vector",
                            Some(p) if p.vend < info.n as usize => "vector+tail",
                            Some(_) => "vector only",
                        })
                        .or_insert(0) += 1;
                        for (k, v) in want[lo..lo + cnt].iter().enumerate() {
                            let vec_part = pl.map_or(false, |p| k >= p.peel && k < p.vend);
                            if v.is_nan() {
                                nans += 1;
                            } else if vec_part && rmax < 1.0 && *v == rmax {
                                at_rmax_vec += 1;
                            } else if *v > 0.0 && *v < 1.0 {
                                interior += 1;
                            } else if *v == 1.0 {
                                at_one += 1;
                            } else if *v == 0.0 {
                                zeros += 1;
                            }
                        }
                    }
                    AlphaRoute::Fill { .. } => {
                        fills += 1;
                        fill_floats += cnt;
                    }
                    _ => {}
                }
                outputs.insert((nm.clone(), c.name.clone()), want.iter().map(|v| v.to_bits()).collect());
            }
        }
    }
    // input sensitivity ("mutation of the main inputs"): a field counts as covered when changing it changes the original's output in at least one case
    let mut killed = Vec::new();
    let mut missed = Vec::new();
    for (grp, fields) in [("mutR_", vec!["v88", "v6c", "va4", "s8", "s48", "s30", "n", "ph", "rmax"]), ("mutF_", vec!["roi_form", "f28", "f2c"]), ("mutG_", vec!["f44"])] {
        for f in fields {
            let mut hit = false;
            for ((nm, case), v) in &outputs {
                if nm == &format!("{}{}", grp, f) {
                    if let Some(b) = outputs.get(&(format!("{}base", grp), case.clone())) {
                        if b != v {
                            hit = true;
                        }
                    }
                }
            }
            if hit { killed.push(format!("{}{}", grp, f)) } else { missed.push(format!("{}{}", grp, f)) }
        }
    }
    println!(
        "ENTRY T2 planes (slot 52 0x548e00): sets {} compared {} floats {} (written {}) differing {} max_ulp {} | outside contract {} (port Err on {}) | written ramp samples: interior {} at 1.0 {} at rmax(<1, vector part) {} at 0 {} nan {} | fill cases {} ({} floats) | destination phases {:?} | sections {:?} | input sensitivity {}/{} missed {:?} | first: {}",
        sets, compared, floats, written, diff, maxulp, outc, outc_ok, interior, at_one, at_rmax_vec, zeros, nans, fills, fill_floats, phs, sec, killed.len(), killed.len() + missed.len(), missed, first
    );
    assert_eq!(outc, outc_ok);
    assert!(compared >= 150);
    assert_eq!(diff, 0, "{}", first);
}
