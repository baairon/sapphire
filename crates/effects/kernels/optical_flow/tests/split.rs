//! The shared partition text equals the arithmetic of the row splitter model it replaces, for every counted thread count.
use optical_flow::split::split;

fn old_chunk(total: i32, threads: i32, tid: i32) -> (i32, i32) {
    let t = threads.max(1);
    if total < t {
        let c = if tid < total { 1 } else { 0 };
        return (tid.wrapping_mul(c), c);
    }
    let q = total / t;
    let start = q.wrapping_mul(tid);
    let count = if tid == t - 1 { total.wrapping_sub(start) } else { q };
    (start, count)
}

#[test]
fn shared_partition_equals_the_row_model() {
    let mut n = 0;
    for t in 1..=8 {
        for total in 0..=260 {
            for tid in 0..t {
                assert_eq!(split(total, t, tid), old_chunk(total, t, tid), "t={t} total={total} tid={tid}");
                n += 1;
            }
        }
    }
    assert!(n > 5000);
}

#[test]
fn one_thread_takes_the_whole_range() {
    for total in 0..64 {
        assert_eq!(split(total, 1, 0), (0, total));
    }
}
