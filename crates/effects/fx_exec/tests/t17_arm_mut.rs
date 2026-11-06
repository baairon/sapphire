//! sweeps that judge the arm mutants 23, 24, 33, 34 of `grad_k`. Each sweep prints a digest line `DIGEST <name> <fnv64>`; a driver script runs this test
//! in the switch build (feature drw2_mut) once per mutant id (COV_MUT) and compares digests with the base run. A digest that differs from the base KILLS the
//! mutant on that sweep; equal digests mean the mutant is not observable on the sweep (equivalent on that domain).
//! These sweeps are NATIVE specification checks: they do not by themselves count as emulator-judged kills; the judge for each sweep is named in MUT_TABLE.tsv.
use fx_exec::kernels::run_any;
use fx_exec::kernels2::Out;
use fx_exec::*;

fn fnv(h: &mut u64, s: &str) {
    for b in s.bytes() {
        *h ^= b as u64;
        *h = h.wrapping_mul(0x100000001b3);
    }
}
fn lcg_img(w: usize, h: usize, planes: usize, seed: u64) -> Image {
    let mut s = seed;
    let mut v = Vec::new();
    for _ in 0..planes {
        v.push((0..w * h).map(|_| {
            s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            0.05 + ((s >> 33) as u32 as f32 / 2147483648.0)
        }).collect::<Vec<f32>>());
    }
    Image::from_planes(v).unwrap()
}
fn outcome(r: Result<Out, ExecError>, h: &mut u64, tagline: &str) {
    fnv(h, tagline);
    match r {
        Ok(Out::Img(i)) => {
            fnv(h, &format!("OK{}", i.channels()));
            for p in &i.planes {
                for v in p {
                    fnv(h, &format!("{:08x}", v.to_bits()));
                }
            }
        }
        Ok(_) => fnv(h, "OKOTHER"),
        Err(e) => fnv(h, &format!("ERR:{}", e).split(':').take(2).collect::<Vec<_>>().join(":")),
    }
}
fn cst(v: f32) -> Arg<'static> {
    Arg::Const { n: 1, lanes: [v.to_bits(); 4] }
}

/// Mutant 23 and the route: the selector can give only 2 or -1 for grad, whatever the operand counts (exhaustive over counts 0..=4, 0 to 6 operands).
#[test]
fn selector_tag_domain_is_2_or_minus_1() {
    if !effects_rtdata::have_tables() {
        return;
    }
    let t = Tables::embedded();
    let (mut n, mut n2) = (0usize, 0usize);
    fn rec(t: &Tables, cur: &mut Vec<i32>, depth: usize, n: &mut usize, n2: &mut usize) {
        let tag = t.select_counts("grad", cur);
        *n += 1;
        assert!(tag == 2 || tag == -1, "tag {} for {:?}", tag, cur);
        if tag == 2 {
            *n2 += 1;
            // select_counts is a prefix matcher: a list shorter than four can give 2 (grad_k then refuses it in need(args, 4)); with four or more operands
            // (the only lists grad_k accepts) tag 2 needs the first four counts to be (1,1,1,1).
            if cur.len() >= 4 {
                assert!(cur[..4] == [1, 1, 1, 1], "tag 2 with >= 4 operands only for (1,1,1,1): {:?}", cur);
            }
        }
        if depth < 6 {
            for c in 0..=4 {
                cur.push(c);
                rec(t, cur, depth + 1, n, n2);
                cur.pop();
            }
        }
    }
    rec(&t, &mut Vec::new(), 0, &mut n, &mut n2);
    eprintln!("selector sweep: {} count tuples, {} give tag 2, the rest -1", n, n2);
    assert!(n2 >= 1);
}

#[test]
fn digest_route_shapes_and_widths() {
    if !effects_rtdata::have_tables() {
        return;
    }
    // ROUTE sweep: tag from the selector (as exec.rs does, refusing tag < 0 before the kernel), widths 1..9, source as 1-plane image or count-1 constant,
    // equal lanes (what fx_eval::lanes_of builds), a spread of host aspects.
    let tables = Tables::embedded();
    let mut h = 0xcbf29ce484222325u64;
    for (w, hh) in [(1usize, 4usize), (2, 2), (3, 3), (8, 4), (9, 2)] {
        let src = lcg_img(w, hh, 1, 3 + w as u64);
        for srcconst in [false, true] {
            for asp in [1.0f32, 2.0, 0.5] {
                for (amt, gain, sub) in [(0.4f32, 0.5f32, 1.0f32), (0.0, 0.5, 1.0), (-1.4129, 0.25, 0.0)] {
                    let args: Vec<Arg> = vec![if srcconst { cst(0.3) } else { Arg::Img(&src) }, cst(amt), cst(gain), cst(sub)];
                    let counts: Vec<i32> = args.iter().map(|a| a.count()).collect();
                    let tag = tables.select_counts("grad", &counts);
                    let mut kp = KParams::standin();
                    kp.host_aspect = asp;
                    let line = format!("w{} h{} sc{} a{} {} {} {}", w, hh, srcconst, asp, amt, gain, sub);
                    if tag < 0 {
                        fnv(&mut h, &(line + " REFUSED-BY-EXEC"));
                        continue;
                    }
                    outcome(run_any("grad", tag, &args, w * hh, (w, hh), &kp), &mut h, &line);
                }
            }
        }
    }
    println!("DIGEST route {:016x}", h);
}

#[test]
fn digest_direct_tags_probe() {
    // PROBE (not reachable through exec): direct run_any with tags other than 2.
    let src = lcg_img(8, 4, 1, 9);
    let args = [Arg::Img(&src), cst(0.4), cst(0.5), cst(1.0)];
    let mut h = 0xcbf29ce484222325u64;
    for tag in [-1, 0, 1, 2, 3, 4] {
        outcome(run_any("grad", tag, &args, 32, (8, 4), &KParams::standin()), &mut h, &format!("tag{}", tag));
    }
    println!("DIGEST tags {:016x}", h);
}

#[test]
fn digest_multi_plane_source_probe() {
    // PROBE for mutant 33: a source with 1..4 planes, called with tag 2 directly (the selector never gives tag 2 for count != 1).
    let mut h = 0xcbf29ce484222325u64;
    for planes in 1..=4usize {
        let src = lcg_img(8, 4, planes, 20 + planes as u64);
        let args = [Arg::Img(&src), cst(0.4), cst(0.5), cst(1.0)];
        outcome(run_any("grad", 2, &args, 32, (8, 4), &KParams::standin()), &mut h, &format!("planes{}", planes));
    }
    println!("DIGEST planes {:016x}", h);
}

#[test]
fn digest_unequal_lanes_probe() {
    // PROBE for mutant 34: a count-1 constant whose lane 1 differs from lane 0 (fx_eval::lanes_of never builds one: it replicates the last element).
    let src = lcg_img(8, 4, 1, 31);
    let mut h = 0xcbf29ce484222325u64;
    for (l0, l1) in [(0.4f32, 0.4f32), (0.4, 0.9), (0.0, 0.7), (-1.0, 2.0)] {
        let amt = Arg::Const { n: 1, lanes: [l0.to_bits(), l1.to_bits(), l1.to_bits(), l1.to_bits()] };
        let args = [Arg::Img(&src), amt, cst(0.5), cst(1.0)];
        outcome(run_any("grad", 2, &args, 32, (8, 4), &KParams::standin()), &mut h, &format!("lanes{} {}", l0, l1));
    }
    println!("DIGEST lanes {:016x}", h);
}
