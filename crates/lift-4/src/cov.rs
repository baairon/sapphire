//! Branch coverage helper for the parity tests.
//!
//! `br(cond)` returns `cond`. With the `cov` feature it also records, per call site (file and line), how often
//! the condition was true and false on the current thread. `hit()` marks a call site that is only ever reached
//! (match arms, jump-table cases). Keep one `br` or `hit` call per source line.

#[cfg(feature = "cov")]
mod imp {
    use std::cell::RefCell;
    use std::collections::BTreeMap;

    thread_local! {
        static HITS: RefCell<BTreeMap<(&'static str, u32), (u32, u32)>> = RefCell::new(BTreeMap::new());
    }

    #[track_caller]
    pub fn br(c: bool) -> bool {
        let l = std::panic::Location::caller();
        HITS.with(|h| {
            let mut h = h.borrow_mut();
            let e = h.entry((l.file(), l.line())).or_insert((0, 0));
            if c {
                e.0 += 1;
            } else {
                e.1 += 1;
            }
        });
        c
    }

    #[track_caller]
    pub fn hit() {
        br(true);
    }

    /// Prints one line per recorded site: `COV <label> <file>:<line> T=<n> F=<n>` and clears the table.
    pub fn report(label: &str) {
        HITS.with(|h| {
            for ((file, line), (t, f)) in h.borrow().iter() {
                println!("COV {} {}:{} T={} F={}", label, file.replace('\\', "/"), line, t, f);
            }
            h.borrow_mut().clear();
        });
    }
}

#[cfg(feature = "cov")]
pub use imp::{br, hit, report};

#[cfg(not(feature = "cov"))]
#[inline(always)]
pub fn br(c: bool) -> bool {
    c
}

#[cfg(not(feature = "cov"))]
#[inline(always)]
pub fn hit() {}

#[cfg(not(feature = "cov"))]
pub fn report(_label: &str) {}
