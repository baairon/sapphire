//! The row splitter of the composed host: the one ported partition text of the class (`warp_log_gradient::partition`), called inside
//! a worker pool as the original calls it. Label SPLIT-MODEL: workers run one after the other (SERIAL-WORKERS).
use warp_log_gradient::{partition, Pool, Threads};

/// Chunk of worker `tid` out of `threads` over `total` rows or elements: (start, count).
pub fn split(total: i32, threads: i32, tid: i32) -> (i32, i32) {
    let t = threads.max(1);
    let mut pool = Pool { g_cell_b: 1, g_cell_c: 1, g_cell_d: 1, wpg2_s33_pool_b: 1, wpg2_s33_pool_a: 1, ..Pool::default() };
    let th = Threads { nthr: t, nthr2: t, tid };
    let mut ev = Vec::new();
    partition(total, &mut pool, &th, &mut ev).expect("a thread count of at least one cannot divide by zero")
}
