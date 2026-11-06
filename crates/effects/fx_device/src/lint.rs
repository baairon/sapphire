//! The kernel source lint: what a device source may and may not contain.
// Provenance: adapted from the lint of twixtor plugins/retime-engine/src/device/tests.rs (8087d06), with the fma rule added.
//!
//! Banned: `mad` and its variants (they fuse unless told not to and are not specified to the bit), `native_` and
//! `half_` functions, `pow log exp fmin fmax mix` (results not specified to the bit, or different NaN and
//! signed zero behaviour from the x86 select the CPU kernels use), decimal float literals, `while` loops, `for`
//! loops without a constant bound, and `FP_CONTRACT` anywhere but the first line of the prelude.
//!
//! `fma()` is allowed only in a kernel registered as mirroring a CPU `mul_add` (the OpenCL `fma` is correctly rounded,
//! as `mul_add` is): the registry flag and the marker comment `MIRRORS_MUL_ADD:` in the text must both be present.

/// `source` without its `/* */` and `//` comments (kernel sources have no string literals).
pub fn strip_comments(source: &str) -> String {
    let mut out = String::with_capacity(source.len());
    let b = source.as_bytes();
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'/' && b.get(i + 1) == Some(&b'*') {
            let end = source[i + 2..].find("*/").map(|e| i + 2 + e + 2).unwrap_or(b.len());
            out.push(' ');
            i = end;
        } else if b[i] == b'/' && b.get(i + 1) == Some(&b'/') {
            let end = source[i..].find('\n').map(|e| i + e).unwrap_or(b.len());
            i = end;
        } else {
            out.push(b[i] as char);
            i += 1;
        }
    }
    out
}

fn is_ident(c: u8) -> bool {
    c.is_ascii_alphanumeric() || c == b'_'
}

fn identifiers(code: &str) -> Vec<&str> {
    let b = code.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < b.len() {
        if b[i].is_ascii_digit() {
            while i < b.len() && (is_ident(b[i]) || b[i] == b'.' || ((b[i] == b'+' || b[i] == b'-') && matches!(b[i - 1], b'e' | b'E' | b'p' | b'P'))) {
                i += 1;
            }
        } else if is_ident(b[i]) {
            let start = i;
            while i < b.len() && is_ident(b[i]) {
                i += 1;
            }
            out.push(&code[start..i]);
        } else {
            i += 1;
        }
    }
    out
}

fn numbers(code: &str) -> Vec<&str> {
    let b = code.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < b.len() {
        let starts = b[i].is_ascii_digit() || (b[i] == b'.' && b.get(i + 1).is_some_and(u8::is_ascii_digit));
        let after_ident = i > 0 && is_ident(b[i - 1]);
        if starts && !after_ident {
            let start = i;
            i += 1;
            while i < b.len() && (is_ident(b[i]) || b[i] == b'.' || ((b[i] == b'+' || b[i] == b'-') && matches!(b[i - 1], b'e' | b'E' | b'p' | b'P'))) {
                i += 1;
            }
            out.push(&code[start..i]);
        } else {
            i += 1;
        }
    }
    out
}

fn is_decimal_float(n: &str) -> bool {
    let lower = n.to_ascii_lowercase();
    if lower.starts_with("0x") {
        return false;
    }
    lower.contains('.') || lower.contains('e') || lower.ends_with('f') || lower.ends_with('h')
}

const FORBIDDEN: &[&str] = &[
    "mad", "mad24", "mad_hi", "mad_sat", "pow", "pown", "powr", "log", "log2", "log10", "log1p", "logb", "exp", "exp2", "exp10", "expm1", "fmin", "fmax", "mix",
];
const FORBIDDEN_PREFIXES: &[&str] = &["native_", "half_"];

/// Every lint finding of one source. `mirrors_mul_add` is the registry flag of the kernel.
pub fn lint(source: &str, mirrors_mul_add: bool) -> Vec<String> {
    let code = strip_comments(source);
    let mut out = Vec::new();
    let fma_ok = mirrors_mul_add && source.contains("MIRRORS_MUL_ADD:");
    for id in identifiers(&code) {
        if FORBIDDEN.contains(&id) || FORBIDDEN_PREFIXES.iter().any(|p| id.starts_with(p)) {
            out.push(format!("forbidden `{id}`"));
        }
        if id == "fma" && !fma_ok {
            out.push("`fma` outside a kernel marked as mirroring a CPU mul_add".into());
        }
    }
    for n in numbers(&code) {
        if is_decimal_float(n) {
            out.push(format!("decimal float literal `{n}`"));
        }
    }
    if code.contains("while") {
        out.push("a while loop (unbounded)".into());
    }
    let mut rest = code.as_str();
    while let Some(p) = rest.find("for (") {
        let head = &rest[p + 5..];
        let close = head.find(')').unwrap_or(head.len());
        let parts: Vec<&str> = head[..close].split(';').collect();
        let bounded = parts.len() == 3 && {
            let cond = parts[1].trim();
            let bound = cond.rsplit(['<', '>']).next().unwrap_or("").trim_start_matches('=').trim();
            !bound.is_empty() && bound.bytes().all(|c| c.is_ascii_digit())
        };
        if !bounded {
            out.push(format!("a loop without a constant bound: `for ({}`", &head[..close.min(60)]));
        }
        rest = &head[close.min(head.len())..];
    }
    if code.contains("FP_CONTRACT") && !code.starts_with("#pragma OPENCL FP_CONTRACT OFF") {
        out.push("FP_CONTRACT outside the prelude's first line".into());
    }
    out
}

/// Lints every registered source; the findings as `name: finding`.
pub fn lint_all() -> Vec<String> {
    let mut out = Vec::new();
    for s in crate::kernels::SOURCES {
        for f in lint(s.text, s.mirrors_mul_add) {
            out.push(format!("{}: {f}", s.name));
        }
    }
    // the whole program contains the fused kernels: allowed exactly when some registered source is flagged and marked
    let any_fused = crate::kernels::SOURCES.iter().any(|s| s.mirrors_mul_add);
    for f in lint(&crate::kernels::device_program(), any_fused) {
        out.push(format!("device_program: {f}"));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernels;

    #[test]
    fn the_prelude_starts_with_fp_contract_off() {
        assert_eq!(crate::prelude::PRELUDE.lines().next(), Some("#pragma OPENCL FP_CONTRACT OFF"));
        assert!(kernels::device_program().starts_with("#pragma OPENCL FP_CONTRACT OFF\n"));
        assert_eq!(crate::prelude::PRELUDE.matches("FP_CONTRACT").count(), 1);
        for s in kernels::SOURCES.iter().filter(|s| s.name != "prelude::PRELUDE") {
            assert!(!s.text.contains("FP_CONTRACT"), "{} must not touch FP_CONTRACT", s.name);
            assert!(!s.text.contains("#pragma"), "{}: pragmas belong to the prelude", s.name);
        }
    }

    #[test]
    fn every_kernel_source_passes_the_lint() {
        assert!(!kernels::SOURCES.is_empty());
        assert_eq!(lint_all(), Vec::<String>::new());
    }

    #[test]
    fn every_source_constant_is_registered() {
        let file = include_str!("kernels.rs");
        for line in file.lines() {
            if let Some(rest) = line.strip_prefix("pub const ") {
                if rest.contains(": &str = r#\"") {
                    let name = rest.split(':').next().unwrap();
                    assert!(
                        kernels::SOURCES.iter().any(|s| s.name.ends_with(&format!("::{name}"))),
                        "{name} is not listed in SOURCES"
                    );
                }
            }
        }
    }

    #[test]
    fn the_lint_catches_what_it_forbids() {
        let bad = [
            ("float y = mad(a, b, c);", "forbidden `mad`"),
            ("float y = native_exp(a);", "forbidden `native_exp`"),
            ("float y = half_sqrt(a);", "forbidden `half_sqrt`"),
            ("float y = pow(a, b);", "forbidden `pow`"),
            ("float y = log2(a);", "forbidden `log2`"),
            ("float y = exp(a);", "forbidden `exp`"),
            ("float y = fmin(a, b);", "forbidden `fmin`"),
            ("float y = fmax(a, b);", "forbidden `fmax`"),
            ("float y = mix(a, b, t);", "forbidden `mix`"),
            ("float y = x * 0.5f;", "decimal float literal `0.5f`"),
            ("float y = x * .5;", "decimal float literal `.5`"),
            ("float y = x * 1e3f;", "decimal float literal `1e3f`"),
            ("float y = x * 2.0;", "decimal float literal `2.0`"),
            ("while (x) { }", "a while loop (unbounded)"),
            ("for (int i = 0; i < n; i++) { }", "a loop without a constant bound: `for (int i = 0; i < n; i++`"),
            ("#pragma OPENCL FP_CONTRACT ON", "FP_CONTRACT outside the prelude's first line"),
            ("float y = fma(a, b, c);", "`fma` outside a kernel marked as mirroring a CPU mul_add"),
        ];
        for (src, want) in bad {
            assert!(lint(src, false).iter().any(|f| f == want), "`{src}` gave {:?}", lint(src, false));
        }
        let good = "/* 0.5 mad pow */ // fma 1.0\nint expected = 0x1p-1f > 0 ? 128 : 255; float mixed_up = 0x1.8p+1f * 0x0p+0f; \
                    for (int s = 0; s < 64; s++) { } for (int s = 3; s >= 0; s--) { }";
        assert_eq!(lint(good, false), Vec::<String>::new());
    }

    #[test]
    fn fma_needs_both_the_registry_flag_and_the_marker() {
        let marked = "/* MIRRORS_MUL_ADD: interp::render */ float y = fma(a, b, c);";
        assert_eq!(lint(marked, true), Vec::<String>::new());
        assert!(!lint(marked, false).is_empty(), "the flag is missing");
        assert!(!lint("float y = fma(a, b, c);", true).is_empty(), "the marker is missing");
        assert!(!lint("float y = mad(a, b, c);", true).is_empty(), "mad is never allowed");
    }

    #[test]
    fn the_build_options_are_exactly_the_strict_ones() {
        assert_eq!(kernels::BUILD_OPTIONS, "-cl-std=CL1.2 -cl-fp32-correctly-rounded-divide-sqrt");
        for relaxing in ["fast-relaxed-math", "mad-enable", concat!("un", "safe-math"), "finite-math", "no-signed-zeros", "denorms-are-zero", "-w"] {
            assert!(!kernels::BUILD_OPTIONS.contains(relaxing), "{relaxing}");
        }
    }
}
