//! docs/cli.md against the code it describes.
//!
//! The page lists every refusal label, flag and environment variable of the command line. These
//! tests fail when the code gains one the page lacks, and when the page names a label or a message
//! stem the code no longer has.
//!
//! A refusal label is an upper case word of two or more parts joined by hyphens, such as
//! `ENV-SWITCH-REFUSED`, found inside a string literal of non-test code. The labels of the stage
//! layer (`crates/neutral-handoff/src`) and of device selection (`fx_exec/src/backend.rs`,
//! `fx_device/src`) are all required on the page. Of the `fx_exec` and `fx_eval` route checks,
//! which number in the hundreds and are internal to the renderer, the page names the ones a
//! stage run reaches (`ROUTE_LABELS` below) and describes the rest as a class.
//!
//! `CLI_DOC_PATH` points the tests at another copy of the page.

use std::fs;
use std::path::{Path, PathBuf};

/// Route checks of the renderer that a stage run reaches; each must be in the code and on the page.
const ROUTE_LABELS: &[&str] = &[
    "ENV-SWITCH-REFUSED",
    "HOST-VALUE-OFF-GRID",
    "POPUP-ANSWER-OUT-OF-RANGE",
    "SOURCE-VARIANT-MISMATCH",
    "TICKMAP-POINT-HOST-INPUT-UNSOURCED",
    "TICKMAP-HOST-FIELD-UNSOURCED",
    "PARAM-UNITS-NOT-JUDGED",
    "FPS-UNGIVEN",
];

/// Hyphenated upper case words in string literals that are not refusal labels.
const NOT_LABELS: &[&str] = &["ARIB-STD-B67", "UTF-8", "SHA-256"];

/// Environment variables read through the process environment by the shipped code, outside `SAPPHIRE_*`.
const OTHER_VARIABLES: &[&str] = &[
    "MEDIA_FFMPEG_PATH",
    "MEDIA_FFPROBE_PATH",
    "FFMPEG",
    "FFPROBE",
    "VFX_PATH",
    "HANDOFF_TWIXTOR_DIR",
    "CARGO_TARGET_DIR",
    "PATH",
    "PATHEXT",
    "LOCALAPPDATA",
    "ProgramFiles",
    "XDG_CACHE_HOME",
    "HOME",
];

/// Whether `doc` holds `token` as a whole word (no letter, digit, `_` or `-` on either side).
fn has_token(doc: &str, token: &str) -> bool {
    let word = |c: char| c.is_ascii_alphanumeric() || c == '_' || c == '-';
    doc.match_indices(token).any(|(i, _)| {
        let before = doc[..i].chars().next_back();
        let after = doc[i + token.len()..].chars().next();
        !before.is_some_and(word) && !after.is_some_and(word)
    })
}

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..")
}

fn page() -> String {
    let path = std::env::var_os("CLI_DOC_PATH").map(PathBuf::from).unwrap_or_else(|| root().join("docs").join("cli.md"));
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("cannot read {}: {}", path.display(), e))
}

/// Every `.rs` file under `dir` that is shipped code: no `tests`, `examples`, `target` folder.
fn sources(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(rd) = fs::read_dir(dir) else { return };
    let mut entries: Vec<PathBuf> = rd.filter_map(|e| e.ok().map(|e| e.path())).collect();
    entries.sort();
    for p in entries {
        let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if p.is_dir() {
            if !matches!(name, "tests" | "examples" | "benches" | "target" | "vectors" | "ext") {
                sources(&p, out);
            }
        } else if name.ends_with(".rs") {
            out.push(p);
        }
    }
}

/// The text of a source file without its `#[cfg(test)]` items: each starts at a column 0 attribute line
/// and ends at the first following line that is a column 0 `}` (or at a header that ends in `;`).
fn shipped(path: &Path) -> String {
    let text = fs::read_to_string(path).unwrap_or_default();
    let mut out = String::new();
    let mut lines = text.lines();
    while let Some(line) = lines.next() {
        if line.trim_end() == "#[cfg(test)]" {
            for item in lines.by_ref() {
                if item.trim_end().ends_with(';') && !item.starts_with(' ') || item == "}" {
                    break;
                }
            }
            continue;
        }
        out.push_str(line);
        out.push('\n');
    }
    out
}

/// The contents of the single line string literals of `text`, comment lines skipped.
fn literals(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    for line in text.lines() {
        if line.trim_start().starts_with("//") {
            continue;
        }
        let mut cur: Option<String> = None;
        let mut chars = line.chars();
        while let Some(c) = chars.next() {
            match (&mut cur, c) {
                (Some(s), '\\') => {
                    s.push(c);
                    if let Some(n) = chars.next() {
                        s.push(n);
                    }
                }
                (Some(_), '"') => out.push(cur.take().unwrap()),
                (Some(s), _) => s.push(c),
                (None, '"') => cur = Some(String::new()),
                (None, _) => {}
            }
        }
    }
    out
}

fn is_label(token: &str) -> bool {
    let parts: Vec<&str> = token.split('-').collect();
    parts.len() >= 2
        && parts.iter().all(|p| !p.is_empty() && p.chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_'))
        && parts[0].chars().next().is_some_and(|c| c.is_ascii_uppercase())
        && !NOT_LABELS.contains(&token)
}

/// The refusal labels inside the string literals of `text`.
fn labels_in(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    for lit in literals(text) {
        for token in lit.split(|c: char| !(c.is_ascii_alphanumeric() || c == '_' || c == '-')) {
            let token = token.trim_matches('-');
            if is_label(token) && !out.iter().any(|t| t == token) {
                out.push(token.to_string());
            }
        }
    }
    out
}

fn labels_of(files: &[PathBuf]) -> Vec<(String, PathBuf)> {
    let mut out: Vec<(String, PathBuf)> = Vec::new();
    for f in files {
        for l in labels_in(&shipped(f)) {
            if !out.iter().any(|(t, _)| *t == l) {
                out.push((l, f.clone()));
            }
        }
    }
    out
}

fn stage_layer_files() -> Vec<PathBuf> {
    let r = root().join("crates");
    let mut files = Vec::new();
    sources(&r.join("neutral-handoff").join("src"), &mut files);
    sources(&r.join("effects").join("fx_device").join("src"), &mut files);
    files.push(r.join("effects").join("fx_exec").join("src").join("backend.rs"));
    files
}

fn all_code() -> String {
    let mut files = Vec::new();
    sources(&root().join("crates"), &mut files);
    files.iter().map(|f| fs::read_to_string(f).unwrap_or_default()).collect::<Vec<_>>().join("\n")
}

#[test]
fn every_refusal_label_of_the_stage_layer_is_on_the_page() {
    let doc = page();
    let found = labels_of(&stage_layer_files());
    assert!(found.len() >= 6, "the scan found {} labels, expected at least 6: {:?}", found.len(), found);
    let missing: Vec<String> = found.iter().filter(|(l, _)| !has_token(&doc, l)).map(|(l, f)| format!("{} ({})", l, f.display())).collect();
    assert!(missing.is_empty(), "docs/cli.md lacks these refusal labels of the code: {:?}", missing);
}

#[test]
fn the_route_labels_a_stage_reaches_are_in_the_code_and_on_the_page() {
    let doc = page();
    let mut files = Vec::new();
    sources(&root().join("crates").join("effects").join("fx_exec").join("src"), &mut files);
    sources(&root().join("crates").join("effects").join("fx_eval").join("src"), &mut files);
    let in_code: Vec<String> = labels_of(&files).into_iter().map(|(l, _)| l).collect();
    let mut problems = Vec::new();
    for l in ROUTE_LABELS {
        if !in_code.iter().any(|c| c == l) {
            problems.push(format!("{} is no longer a label of fx_exec or fx_eval", l));
        }
        if !has_token(&doc, l) {
            problems.push(format!("{} is not on docs/cli.md", l));
        }
    }
    assert!(problems.is_empty(), "{:?}", problems);
}

#[test]
fn every_label_in_the_label_table_exists_in_the_code() {
    let doc = page();
    let code = all_code();
    let mut rows = 0;
    let mut stale = Vec::new();
    for line in doc.lines() {
        let Some(rest) = line.strip_prefix("| `") else { continue };
        let Some(end) = rest.find('`') else { continue };
        let token = &rest[..end];
        if is_label(token) {
            rows += 1;
            if !code.contains(token) {
                stale.push(token.to_string());
            }
        }
    }
    assert!(rows >= 10, "the label table has {} rows", rows);
    assert!(stale.is_empty(), "labels on docs/cli.md that no code has: {:?}", stale);
}

#[test]
fn every_message_stem_in_the_message_table_exists_in_the_code() {
    let doc = page();
    let code = all_code();
    let mut in_table = false;
    let mut rows = 0;
    let mut stale = Vec::new();
    for line in doc.lines() {
        if line.starts_with("| Message starts with ") {
            in_table = true;
            continue;
        }
        if !in_table {
            continue;
        }
        if !line.starts_with('|') {
            break;
        }
        let Some(rest) = line.strip_prefix("| `") else { continue };
        let Some(end) = rest.find('`') else { continue };
        rows += 1;
        if !code.contains(&rest[..end]) {
            stale.push(rest[..end].to_string());
        }
    }
    assert!(rows >= 15, "the message table has {} rows", rows);
    assert!(stale.is_empty(), "message stems on docs/cli.md that no code has: {:?}", stale);
}

#[test]
fn every_flag_of_the_command_line_is_on_the_page() {
    let doc = page();
    let cli = shipped(&root().join("crates").join("neutral-handoff").join("src").join("cli.rs"));
    let mut flags: Vec<String> = Vec::new();
    for lit in literals(&cli) {
        if lit.starts_with("--") && lit.len() > 2 && lit[2..].chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-') && !flags.contains(&lit) {
            flags.push(lit);
        }
    }
    assert!(flags.len() >= 20, "the scan found {} flags: {:?}", flags.len(), flags);
    let missing: Vec<&String> = flags.iter().filter(|f| !has_token(&doc, f)).collect();
    assert!(missing.is_empty(), "docs/cli.md lacks these flags of cli.rs: {:?}", missing);
}

#[test]
fn every_environment_variable_the_code_reads_is_on_the_page() {
    let doc = page();
    let mut files = Vec::new();
    sources(&root().join("crates"), &mut files);
    let mut names: Vec<String> = Vec::new();
    for f in &files {
        for lit in literals(&shipped(f)) {
            for token in lit.split(|c: char| !(c.is_ascii_alphanumeric() || c == '_')) {
                if token.starts_with("SAPPHIRE_") && token.len() > 9 && token.chars().all(|c| c.is_ascii_uppercase() || c == '_') && !names.iter().any(|n| n == token) {
                    names.push(token.to_string());
                }
            }
        }
    }
    assert!(names.len() >= 9, "the scan found {} SAPPHIRE_ variables: {:?}", names.len(), names);
    let mut missing: Vec<String> = names.iter().filter(|n| !has_token(&doc, n)).cloned().collect();
    missing.extend(OTHER_VARIABLES.iter().filter(|n| !has_token(&doc, n)).map(|n| n.to_string()));
    // the fenced switches of the 'ae32' route
    let full = fs::read_to_string(root().join("crates").join("effects").join("fx_exec").join("src").join("fxrun.rs")).unwrap_or_default();
    let start = full.find("const ENV_SWITCHES").expect("ENV_SWITCHES");
    let end = start + full[start..].find("];").expect("end of ENV_SWITCHES");
    let body: String = full[start..end].lines().filter(|l| !l.trim_start().starts_with("//")).collect::<Vec<_>>().join("\n");
    let switches = literals(&body);
    assert!(switches.len() >= 40, "the scan found {} fenced switches", switches.len());
    missing.extend(switches.iter().filter(|n| !has_token(&doc, n)).cloned());
    assert!(missing.is_empty(), "docs/cli.md lacks these environment variables: {:?}", missing);
}
