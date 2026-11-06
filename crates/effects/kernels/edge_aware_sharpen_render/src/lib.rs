//! Std-only port of the CPU render entry (slot 52, AVX2 body) of edge_aware_sharpen, render flag 0 route.
//!
//! Memory model: one f32 arena; every plane is `w*h` floats (stride == width, ctx[8] == 0 of the ROI record), a plane pointer is its element offset.
//! Callees: the job jobs core (mode 4) with powf and the pass jobs.
//! The plane services (fill, copy, scale, add) are PLANE-SERVICE MODELS judged against the real code in the emulator.
//! The pass wrappers sit on the flag != 0 arms and are NOT ported (refused with FlagNotPorted).
//! HOST inputs (explicit): the plane count (vcall `[[this]]`), the partition service (the port runs each job on the whole range), the this fields.

use edge_aware_sharpen_weights as j2;
pub mod judge_keys;
use edge_aware_sharpen_jobs as j3;
use edge_aware_sharpen_blur_core::{core_mode4, Args, Host};
use edge_aware_sharpen_box_filter as pj;

/// MEASURED-DYLIB constant (captured): 2.125f.
pub const MIX_C: f32 = 2.125;
/// MEASURED-DYLIB : double 1e-10, and its negation .
pub const LOW: f64 = f64::from_bits(0x3ddb_7cdf_d9d7_bdbb);

#[derive(Debug, Clone, PartialEq)]
pub enum DErr {
    /// tripwire: an access to an input plane id that is NO_PLANE (index 3 at nch 3).
    Tripwire(usize),
    FlagNotPorted(u8),
    NScope(i32),
    Iter(i32),
    Core(String),
    Job(String),
}

/// the id of an absent input plane (the original's NULL table entry). Any read of it is a Tripwire error.
pub const NO_PLANE: usize = usize::MAX;

/// The fields of `this` that the route reads (floats as read by the listing).
#[derive(Debug, Clone)]
pub struct This {
    pub n: i32,          // vcall [[this]]
    pub sig_byte: bool,  // [this] != 0: sigma0 float at, no curve plane
    pub sigma0: f32,     // [this]
    pub mode: f32,       // [this] (converted by vcvttss2si)
    pub variant: f32,    // [this]
    pub bcflag: f32,     // [this]
    pub iters: f32,      // [this]
    pub scale: f32,      // [this]
    pub output_a: f32,       // [this]
    pub arr: [f32; 11],  // [this + 4k]: per-iteration values k = 0..6, per-channel gains at k = 7 + ch
}

#[derive(Debug, Clone, Copy)]
pub struct Cx {
    pub cw: i32,    // ctx[0x20]
    pub w: i32,     // ctx[0x28]
    pub h: i32,     // ctx[0x2c]
    pub kcell: f32, // ctx[0x34]
}

/// Plane ids (arena element offsets).
#[derive(Debug, Clone)]
pub struct Ids {
    pub inp: [usize; 4],
    pub t: Vec<usize>, // 3n + 5 entries
    pub out: Vec<usize>,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Calls {
    /// pool-run job addresses in call order (rsi, dispatcher or job entry, listing)
    pub pools: Vec<u32>,
    /// branch bits: zero-radius branch taken count, core branch taken count
    pub zero_br: u32,
    pub core_br: u32,
    pub rec: pj::Rec,
}

fn cvtt32(x: f32) -> i32 {
    j3::cvtt32(x)
}

struct Hn<'a> {
    mem: &'a mut Vec<f32>,
    w: usize,
    h: usize,
    calls: &'a mut Calls,
}

fn copy_plane(mem: &mut [f32], dst: usize, src: usize, n: usize) {
    if dst != src {
        mem.copy_within(src..src + n, dst);
    }
}

impl<'a> Host for Hn<'a> {
    fn powf(&mut self, x: f32, y: f32) -> f32 {
        libm_powf::powf(x, y)
    }
    fn scratch_init(&mut self, _this: u64, _bytes: i64, _ecx: u32, _r8d: u32, _g0: u64, _g1: u64, _hx: &[f32], _hy: &[f32]) -> u64 {
        1
    }
    fn pool_run(&mut self, _this: u64, job: u32, radius: u32, src: u64, dst: u64, aux: u64) {
        self.calls.pools.push(job);
        let jb = pj::Job { w: self.w as i32, h: self.h as i32, radius: f32::from_bits(radius), src: src as usize, dst: dst as usize, aux: aux as usize };
        if job == edge_aware_sharpen_blur_core::JOB_X {
            pj::job_x(self.mem, &jb, 0, self.h as i32, &mut self.calls.rec);
        } else {
            pj::job_y(self.mem, &jb, 0, self.w as i32, &mut self.calls.rec);
        }
    }
    fn copy(&mut self, dst: u64, src: u64, _edx: u32, _w0: u64, _w1: u64, _w2: u64) {
        let n = self.w * self.h;
        copy_plane(self.mem, dst as usize, src as usize, n);
    }
    fn scratch_free(&mut self, _handle: u64, _g0: u64, _g1: u64) {}
}

/// Slot 52, flag 0 route. `mem` holds all planes; returns the call record.
pub fn slot52(th: &This, cx: &Cx, ids: &Ids, mem: &mut Vec<f32>, flag: u8) -> Result<Calls, DErr> {
    if flag != 0 {
        return Err(DErr::FlagNotPorted(flag));
    }
    let n = th.n;
    if !(1..=4).contains(&n) {
        return Err(DErr::NScope(n));
    }
    let nu = n as usize;
    let (w, h) = (cx.w as usize, cx.h as usize);
    let wh = w * h;
    let mut calls = Calls::default();
    // prologue 
    let mode = cvtt32(th.mode);
    let variant = cvtt32(th.variant);
    let bc = cvtt32(th.bcflag);
    let iters = cvtt32(th.iters);
    let sigma0 = if th.sig_byte { th.sigma0 } else { 0.0f32 };
    let width_f = cx.cw as f32;
    let mut base = (0.125f32 * th.scale) * width_f; // [rbp]
    let dv = (n as f32) * th.output_a; // [rbp]
    // zero fill of each output plane (esi == 0)
    for &o in ids.out.iter().take(nu) {
        mem[o..o + wh].iter_mut().for_each(|x| *x = 0.0);
    }
    // pointer tables (S3): A = inputs, B = t[0..n), C = t[n..2n), D = t[2n..3n)
    let in_ids: Vec<usize> = ids.inp.iter().take(nu).copied().collect();
    if let Some(k) = in_ids.iter().position(|&i| i == NO_PLANE) {
        return Err(DErr::Tripwire(k));
    }
    let mut pa: Vec<usize> = in_ids.clone();
    let mut pb: Vec<usize> = (0..nu).map(|i| ids.t[i]).collect();
    let pc: Vec<usize> = (0..nu).map(|i| ids.t[nu + i]).collect();
    let pd: Vec<usize> = (0..nu).map(|i| ids.t[2 * nu + i]).collect();
    let (t0, t1) = (ids.t[3 * nu], ids.t[3 * nu + 1]);
    let (wa, wt) = (ids.t[3 * nu + 2], ids.t[3 * nu + 3]);
    let curve = ids.t[3 * nu + 4];
    let flag_cell_b: i32 = if th.sig_byte { 1 } else { 0 };
    if iters > 0 {
        if iters as usize > 7 {
            return Err(DErr::Iter(iters));
        }
        for it in 0..iters as usize {
            let s2 = base / dv;
            // ---- pre-pass: weight planes a (wa) and t (wt)
            {
                let take = |m: &Vec<f32>, id: usize| m[id..id + wh].to_vec();
                let mut a = take(mem, wa);
                let mut t = take(mem, wt);
                calls.pools.push(if variant != 0 { judge_keys::POOL_A } else { judge_keys::POOL_B });
                if variant != 0 {
                    let c = j2::Ctx { w: w as i32, s: h as i32, flag_cell_b, f0: s2, f4: base, f8: s2, fc: base, esh2_job_ctx_a: sigma0, k: cx.kcell };
                    for k in 0..2 {
                        if ids.inp[k] == NO_PLANE {
                            return Err(DErr::Tripwire(k));
                        }
                    }
                    let (p0, p1) = (take(mem, ids.inp[0]), take(mem, ids.inp[1]));
                    let cur = if flag_cell_b == 0 { take(mem, curve) } else { Vec::new() };
                    let mut b = j2::Bufs { p0: &p0, p1: &p1, cur: &cur, a: &mut a, t: &mut t };
                    j2::job_1(&c, 0, h as i32, 0, &mut b).map_err(|e| DErr::Job(format!("{:?}", e)))?;
                } else {
                    let c = j3::Ctx { w: w as i32, s: h as i32, nch: n, flag_cell_b, f0: s2, f4: base, f8: s2, fc: base, esh2_job_ctx_a: sigma0, k: cx.kcell };
                    let chs: Vec<Vec<f32>> = (0..4).map(|k| if ids.inp[k] == NO_PLANE { Vec::new() } else { take(mem, ids.inp[k]) }).collect();
                    let cur = if flag_cell_b == 0 { take(mem, curve) } else { Vec::new() };
                    let mut b = j3::Bufs { ch: [&chs[0], &chs[1], &chs[2], &chs[3]], cur: &cur, a: &mut a, t: &mut t };
                    j3::job_2(&c, 0, h as i32, &mut b).map_err(|e| DErr::Job(format!("{:?}", e)))?;
                }
                mem[wa..wa + wh].copy_from_slice(&a);
                mem[wt..wt + wh].copy_from_slice(&t);
            }
            if bc == 0 {
                calls.pools.push(judge_keys::POOL_C);
                let a = mem[wa..wa + wh].to_vec();
                let t = mem[wt..wt + wh].to_vec();
                let mut p0 = mem[t0..t0 + wh].to_vec();
                let mut p1 = mem[t1..t1 + wh].to_vec();
                j3::job_cell_a(w as i32, h as i32, (0, h as i32), (0, w as i32), &a, &t, &mut p0, &mut p1).map_err(|e| DErr::Job(format!("{:?}", e)))?;
                mem[t0..t0 + wh].copy_from_slice(&p0);
                mem[t1..t1 + wh].copy_from_slice(&p1);
            } else {
                copy_plane(mem, t0, wa, wh);
                copy_plane(mem, t1, wt, wh);
            }
            // ---- per channel
            for ch in 0..nu {
                let zero = LOW > (base as f64) && (base as f64) > -LOW;
                if zero {
                    calls.zero_br += 1;
                    copy_plane(mem, pb[ch], pa[ch], wh);
                } else {
                    calls.core_br += 1;
                    let args = Args { this: 0, mode, p_in: in_ids[ch] as u64, p_b: wa as u64, p_c: pb[ch] as u64, p_d: t0 as u64, p_e: t1 as u64,
                                      roi: [cx.w, cx.h, cx.w, 0], p_f: 0, n: 3, flag: 0, sx: base, sy: base };
                    let mut hn = Hn { mem, w, h, calls: &mut calls };
                    core_mode4(&args, &mut hn).map_err(|e| DErr::Core(format!("{:?}", e)))?;
                }
                // C = B; C = fma(C, -1, 0); C += A; C = tanh-like(C, s0 = m); out += C
                copy_plane(mem, pc[ch], pb[ch], wh);
                for x in mem[pc[ch]..pc[ch] + wh].iter_mut() {
                    *x = x.mul_add(-1.0, 0.0);
                }
                for i in 0..wh {
                    let v = mem[pc[ch] + i] + mem[pa[ch] + i];
                    mem[pc[ch] + i] = v;
                }
                let m = th.arr[it].mul_add(MIX_C, -MIX_C).mul_add(th.arr[7 + ch], MIX_C);
                calls.pools.push(judge_keys::POOL_D);
                j3::job_3(m, 0, wh as i32, &mut mem[pc[ch]..pc[ch] + wh]).map_err(|e| DErr::Job(format!("{:?}", e)))?;
                let o = ids.out[ch];
                for i in 0..wh {
                    let v = mem[o + i] + mem[pc[ch] + i];
                    mem[o + i] = v;
                }
                if pa[ch] == in_ids[ch] {
                    pa[ch] = pb[ch];
                    pb[ch] = pd[ch];
                } else {
                    std::mem::swap(&mut pa[ch], &mut pb[ch]);
                }
            }
            base = base + base;
        }
    }
    // tail: out += A
    for ch in 0..nu {
        let o = ids.out[ch];
        for i in 0..wh {
            let v = mem[o + i] + mem[pa[ch] + i];
            mem[o + i] = v;
        }
    }
    Ok(calls)
}
