//! Test-side plane store over one stored region addressed by handle (the window and handle arithmetic of the old executor).
use optical_flow_solve::body::{Callback, Level, Params, Refused};
use optical_flow_solve::solve::{PlaneStore, RunRefused};

pub const PLANE_FLOATS: usize = 0x4000 / 4;

pub struct World {
    pub pl: Vec<f32>,
    pub origin: u64,
    pub acc: f32,
    pub launches: Vec<Callback>,
}

impl World {
    fn off(&self, h: u64) -> usize {
        (h.wrapping_sub(self.origin) / 4) as usize
    }
}

impl PlaneStore for World {
    fn read(&self, id: u64) -> Vec<f32> {
        let o = self.off(id);
        self.pl[o..o + PLANE_FLOATS].to_vec()
    }
    fn write(&mut self, id: u64, v: &[f32]) {
        let o = self.off(id);
        self.pl[o..o + v.len()].copy_from_slice(v);
    }
    fn align_bytes(&self, id: u64) -> u64 {
        id
    }
    fn launched(&mut self, c: Callback) {
        self.launches.push(c);
    }
}

/// The old entry point over the stored region: `polls` are the abort answers in call order (0 beyond the list).
pub fn run_body(levels: &[Level; 3], idx: i32, p: &Params, polls: &[u32], w: &mut World) -> Result<(), Refused> {
    let mut n = 0usize;
    let mut poll = || {
        let v = polls.get(n).copied().unwrap_or(0);
        n += 1;
        v != 0
    };
    match optical_flow_solve::solve::run_body(&levels[..], idx, p, w, &mut poll) {
        Ok(_) => Ok(()),
        Err(RunRefused::SmallLevel) => Err(Refused::SmallLevel),
        Err(RunRefused::FirstIndex) => panic!("first index outside the level table"),
    }
}
