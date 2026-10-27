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

    thread_local! {
        static BR: RefCell<BTreeMap<u32, (u64, u64)>> = RefCell::new(BTreeMap::new());
    }

    /// Conditional branch of a lifted function at file offset `addr`: records how often it was taken (true) and not taken (false).
    pub fn brk(addr: u32, c: bool) -> bool {
        BR.with(|b| {
            let mut b = b.borrow_mut();
            let e = b.entry(addr).or_insert((0, 0));
            if c {
                e.0 += 1;
            } else {
                e.1 += 1;
            }
        });
        c
    }

    /// Prints `COVB <label> <addr hex> T=<n> F=<n>` per lifted branch executed on this thread and clears the table.
    pub fn report_branches(label: &str) {
        BR.with(|b| {
            for (a, (t, f)) in b.borrow().iter() {
                println!("COVB {} {:x} T={} F={}", label, a, t, f);
            }
            b.borrow_mut().clear();
        });
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
pub use imp::{br, brk, hit, report, report_branches};

#[cfg(not(feature = "cov"))]
#[inline(always)]
pub fn br(c: bool) -> bool {
    c
}

#[cfg(not(feature = "cov"))]
#[inline(always)]
pub fn brk(_addr: u32, c: bool) -> bool {
    c
}

#[cfg(not(feature = "cov"))]
pub fn report_branches(_label: &str) {}

#[cfg(not(feature = "cov"))]
#[inline(always)]
pub fn hit() {}

#[cfg(not(feature = "cov"))]
pub fn report(_label: &str) {}
