//! Row-band parallelism for kernels whose output rows are independent.
//!
//! A band runs the same per-row operations in the same order as the serial loop; bands only decide which
//! thread computes which rows, so the output bytes do not depend on the thread count.

use crate::types::ExecError;

/// Frames with fewer samples than this stay serial (thread start cost exceeds the gain).
pub const MIN_SAMPLES: usize = 1 << 16;

/// The serial threshold: `SAPPHIRE_PAR_MIN` (samples) when set, else `MIN_SAMPLES`.
pub fn min_samples() -> usize {
    std::env::var("SAPPHIRE_PAR_MIN").ok().and_then(|v| v.trim().parse::<usize>().ok()).unwrap_or(MIN_SAMPLES)
}

/// Worker count: `SAPPHIRE_THREADS` when set to a positive integer (1 = serial), else the available parallelism.
pub fn configured_threads() -> usize {
    if let Ok(v) = std::env::var("SAPPHIRE_THREADS") {
        if let Ok(k) = v.trim().parse::<usize>() {
            if k >= 1 {
                return k;
            }
        }
    }
    std::thread::available_parallelism().map(|k| k.get()).unwrap_or(1)
}

/// Threads to use for `rows` rows of `samples` samples in total.
pub fn threads_for(rows: usize, samples: usize) -> usize {
    if samples < min_samples() || rows < 2 {
        return 1;
    }
    configured_threads().min(rows).max(1)
}

/// Run `band(lo, hi)` over contiguous row bands covering `0..rows` and return the band results in row order.
/// With one thread this is a single call `band(0, rows)`.
pub fn row_bands<T, F>(rows: usize, nt: usize, band: F) -> Result<Vec<T>, ExecError>
where
    T: Send,
    F: Fn(usize, usize) -> Result<T, ExecError> + Sync,
{
    if nt <= 1 || rows < 2 {
        return Ok(vec![band(0, rows)?]);
    }
    let nt = nt.min(rows);
    let per = rows / nt;
    let extra = rows % nt;
    let mut ranges = Vec::with_capacity(nt);
    let mut lo = 0;
    for t in 0..nt {
        let hi = lo + per + usize::from(t < extra);
        ranges.push((lo, hi));
        lo = hi;
    }
    let band = &band;
    let results: Vec<Result<T, ExecError>> = std::thread::scope(|s| {
        let hs: Vec<_> = ranges.iter().map(|&(lo, hi)| s.spawn(move || band(lo, hi))).collect();
        hs.into_iter().map(|h| h.join().unwrap_or_else(|_| Err(ExecError::Shape("row band thread panicked".into())))).collect()
    });
    results.into_iter().collect()
}

/// Join per-band plane sets (each `c` planes of `rows_in_band * w` samples) into full planes.
pub fn join_planes(parts: Vec<Vec<Vec<f32>>>, c: usize, n: usize) -> Vec<Vec<f32>> {
    let mut planes: Vec<Vec<f32>> = (0..c).map(|_| Vec::with_capacity(n)).collect();
    for part in parts {
        for (k, p) in part.into_iter().enumerate() {
            planes[k].extend_from_slice(&p);
        }
    }
    planes
}
