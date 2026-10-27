//! Text helpers that reproduce what `tools/apply.py` prints or accepts.
//!
//! - [`shlex_quote`] and [`format_command`]: `shlex.quote` and the `[cmd]` line.
//! - [`shlex_split`]: `shlex.split` (POSIX rules) for the `--stage CMD` flag.
//! - [`format_g`]: `format(x, "g")`, used for the frame rate of an output line.
//! - [`splitext`], [`basename`], [`dirname`]: `os.path` on Windows and POSIX paths alike.

/// `shlex.quote`: the text unchanged when it is made of safe characters, else in single quotes.
pub fn shlex_quote(text: &str) -> String {
    if text.is_empty() {
        return "''".to_string();
    }
    let safe = text
        .bytes()
        .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'_' | b'@' | b'%' | b'+' | b'=' | b':' | b',' | b'.' | b'/' | b'-'));
    if safe {
        text.to_string()
    } else {
        format!("'{}'", text.replace('\'', "'\"'\"'"))
    }
}

/// An argument list as a copy-pasteable command line (`apply.py`'s `fmt_cmd`).
pub fn format_command(argv: &[String]) -> String {
    argv.iter().map(|a| shlex_quote(a)).collect::<Vec<_>>().join(" ")
}

/// `shlex.split(text)` with POSIX rules and no comments.
///
/// Outside quotes a backslash escapes the next character, single quotes keep everything
/// literally, and inside double quotes a backslash only escapes `"` and `\`. An empty pair of
/// quotes gives an empty argument. On Windows a path therefore needs forward slashes, doubled
/// backslashes or single quotes.
pub fn shlex_split(text: &str) -> Result<Vec<String>, String> {
    #[derive(PartialEq)]
    enum State {
        Space,
        Word,
        Single,
        Double,
    }
    let mut args: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut state = State::Space;
    let mut chars = text.chars();
    while let Some(c) = chars.next() {
        match state {
            State::Space | State::Word => match c {
                ' ' | '\t' | '\r' | '\n' => {
                    if state == State::Word {
                        args.push(std::mem::take(&mut current));
                        state = State::Space;
                    }
                }
                '\'' => state = State::Single,
                '"' => state = State::Double,
                '\\' => match chars.next() {
                    Some(next) => {
                        current.push(next);
                        state = State::Word;
                    }
                    None => return Err("No escaped character".to_string()),
                },
                other => {
                    current.push(other);
                    state = State::Word;
                }
            },
            State::Single => {
                if c == '\'' {
                    state = State::Word;
                } else {
                    current.push(c);
                }
            }
            State::Double => match c {
                '"' => state = State::Word,
                '\\' => match chars.next() {
                    Some(next @ ('"' | '\\')) => current.push(next),
                    Some(next) => {
                        current.push('\\');
                        current.push(next);
                    }
                    None => return Err("No escaped character".to_string()),
                },
                other => current.push(other),
            },
        }
    }
    match state {
        State::Single | State::Double => Err("No closing quotation".to_string()),
        State::Word => {
            args.push(current);
            Ok(args)
        }
        State::Space => Ok(args),
    }
}

fn is_space(c: char) -> bool {
    c.is_ascii_whitespace() && c != '\u{c}'
}

fn is_quote(c: char) -> bool {
    c == '"' || c == '\''
}

/// `shlex.split(text, posix=False)` followed by taking the quotes off a word that starts and ends
/// with the same quote, which is how `tools/apply.py` reads a stage command on Windows.
///
/// A backslash is an ordinary character. A word that starts with a quote ends at the matching
/// quote; a quote inside a word is an ordinary character.
pub fn shlex_split_windows(text: &str) -> Result<Vec<String>, String> {
    let chars: Vec<char> = text.chars().collect();
    let mut words: Vec<String> = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        if is_space(chars[i]) {
            i += 1;
            continue;
        }
        let start = i;
        if is_quote(chars[i]) {
            let quote = chars[i];
            i += 1;
            while i < chars.len() && chars[i] != quote {
                i += 1;
            }
            if i >= chars.len() {
                return Err("No closing quotation".to_string());
            }
            i += 1;
        } else {
            while i < chars.len() && !is_space(chars[i]) {
                i += 1;
            }
        }
        words.push(chars[start..i].iter().collect());
    }
    Ok(words
        .into_iter()
        .map(|w| {
            let cs: Vec<char> = w.chars().collect();
            if cs.len() >= 2 && cs[0] == cs[cs.len() - 1] && is_quote(cs[0]) {
                cs[1..cs.len() - 1].iter().collect()
            } else {
                w
            }
        })
        .collect())
}

/// A stage command line as an argument list: Windows rules (a backslash stays a backslash) when
/// `windows`, else POSIX rules. `apply.py` makes the same choice from `os.name`.
pub fn split_command(text: &str, windows: bool) -> Result<Vec<String>, String> {
    if windows {
        shlex_split_windows(text)
    } else {
        shlex_split(text)
    }
}

/// `format(x, "g")`: six significant digits, no trailing zeros, scientific outside 1e-4 to 1e6.
pub fn format_g(x: f64) -> String {
    if x.is_nan() {
        return "nan".to_string();
    }
    if x.is_infinite() {
        return if x > 0.0 { "inf" } else { "-inf" }.to_string();
    }
    if x == 0.0 {
        return "0".to_string();
    }
    let sci = format!("{:.5e}", x);
    let (mantissa, exponent) = sci.split_once('e').expect("scientific notation has an exponent");
    let exponent: i32 = exponent.parse().expect("exponent is an integer");
    if !(-4..6).contains(&exponent) {
        let sign = if exponent < 0 { '-' } else { '+' };
        return format!("{}e{}{:02}", trim_zeros(mantissa), sign, exponent.abs());
    }
    let decimals = (5 - exponent).max(0) as usize;
    trim_zeros(&format!("{:.*}", decimals, x))
}

fn trim_zeros(s: &str) -> String {
    if s.contains('.') {
        s.trim_end_matches('0').trim_end_matches('.').to_string()
    } else {
        s.to_string()
    }
}

fn last_separator(path: &str) -> Option<usize> {
    path.rfind(['/', '\\'])
}

/// `os.path.basename`: everything after the last separator.
pub fn basename(path: &str) -> &str {
    match last_separator(path) {
        Some(i) => &path[i + 1..],
        None => path,
    }
}

/// `os.path.dirname` (without the trailing separator, and `""` when there is none).
pub fn dirname(path: &str) -> &str {
    match last_separator(path) {
        None => "",
        Some(i) => {
            let head = &path[..i + 1];
            let trimmed = head.trim_end_matches(['/', '\\']);
            if trimmed.is_empty() {
                head
            } else {
                trimmed
            }
        }
    }
}

/// `os.path.splitext`: the last dot of the last component starts the extension, leading dots do not.
pub fn splitext(path: &str) -> (&str, &str) {
    let name_start = last_separator(path).map(|i| i + 1).unwrap_or(0);
    let name = &path[name_start..];
    let lead = name.len() - name.trim_start_matches('.').len();
    match name[lead..].rfind('.') {
        Some(i) => {
            let at = name_start + lead + i;
            (&path[..at], &path[at..])
        }
        None => (path, ""),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quote_matches_python() {
        assert_eq!(shlex_quote("abc-1.2/x:y"), "abc-1.2/x:y");
        assert_eq!(shlex_quote(""), "''");
        assert_eq!(shlex_quote("a b"), "'a b'");
        assert_eq!(shlex_quote("it's"), "'it'\"'\"'s'");
        assert_eq!(shlex_quote(r"C:\data\x"), r"'C:\data\x'");
        assert_eq!(shlex_quote("speed_percent=100@0;50@24"), "'speed_percent=100@0;50@24'");
        assert_eq!(shlex_quote("\u{e9}"), "'\u{e9}'");
        assert_eq!(format_command(&["a".into(), "b c".into()]), "a 'b c'");
    }

    #[test]
    fn split_matches_python_shlex() {
        assert_eq!(shlex_split("a b  c").unwrap(), ["a", "b", "c"]);
        assert_eq!(shlex_split("  ").unwrap(), Vec::<String>::new());
        assert_eq!(shlex_split(r#"a "b c" 'd e' f\ g"#).unwrap(), ["a", "b c", "d e", "f g"]);
        assert_eq!(shlex_split(r#"x "" y"#).unwrap(), ["x", "", "y"]);
        assert_eq!(shlex_split(r#""a\"b" "c\d" 'e\f'"#).unwrap(), [r#"a"b"#, r"c\d", r"e\f"]);
        assert_eq!(shlex_split("a'b c'd").unwrap(), ["ab cd"]);
        assert_eq!(shlex_split("C:/tools/vfx.exe apply").unwrap(), ["C:/tools/vfx.exe", "apply"]);
        assert_eq!(shlex_split(r"C:\tools\vfx").unwrap(), ["C:toolsvfx"], "an unquoted backslash escapes");
        assert_eq!(shlex_split(r"'C:\tools\vfx'").unwrap(), [r"C:\tools\vfx"]);
        assert!(shlex_split("a \"b").is_err());
        assert!(shlex_split("a 'b").is_err());
        assert!(shlex_split("a \\").is_err());
    }

    #[test]
    fn windows_splitting_keeps_backslashes_and_strips_outer_quotes() {
        assert_eq!(shlex_split_windows(r#"C:\tools\vfx.exe apply 'a b' "c d" e"#).unwrap(), [r"C:\tools\vfx.exe", "apply", "a b", "c d", "e"]);
        assert_eq!(shlex_split_windows("a'b c'd").unwrap(), ["a'b", "c'd"], "a quote inside a word is a plain character");
        assert_eq!(shlex_split_windows("'a b'c").unwrap(), ["a b", "c"], "a quoted word ends at its closing quote");
        assert_eq!(shlex_split_windows("  ").unwrap(), Vec::<String>::new());
        assert_eq!(shlex_split_windows("x ''").unwrap(), ["x", ""]);
        assert!(shlex_split_windows("a \"b").is_err());
        assert_eq!(split_command("a  b", true).unwrap(), ["a", "b"]);
        assert_eq!(split_command(r"a\ b", false).unwrap(), ["a b"]);
        assert_eq!(split_command(r"a\ b", true).unwrap(), [r"a\", "b"]);
    }

    #[test]
    fn format_g_matches_python() {
        assert_eq!(format_g(24.0), "24");
        assert_eq!(format_g(29.97), "29.97");
        assert_eq!(format_g(30000.0 / 1001.0), "29.97");
        assert_eq!(format_g(52.555), "52.555");
        assert_eq!(format_g(0.0), "0");
        assert_eq!(format_g(1234567.0), "1.23457e+06");
        assert_eq!(format_g(100000.0), "100000");
        assert_eq!(format_g(0.0001), "0.0001");
        assert_eq!(format_g(0.00001), "1e-05");
        assert_eq!(format_g(0.5), "0.5");
        assert_eq!(format_g(120.0), "120");
    }

    #[test]
    fn path_pieces_match_os_path() {
        assert_eq!(splitext("a/b.c/d"), ("a/b.c/d", ""));
        assert_eq!(splitext("clip.mp4"), ("clip", ".mp4"));
        assert_eq!(splitext("a.b.mkv"), ("a.b", ".mkv"));
        assert_eq!(splitext(".bashrc"), (".bashrc", ""));
        assert_eq!(splitext(r"C:\x\clip.MOV"), (r"C:\x\clip", ".MOV"));
        assert_eq!(splitext("noext"), ("noext", ""));
        assert_eq!(basename(r"C:\x\clip.mp4"), "clip.mp4");
        assert_eq!(basename("a/b/c.mp4"), "c.mp4");
        assert_eq!(basename("c.mp4"), "c.mp4");
        assert_eq!(dirname("c.mp4"), "");
        assert_eq!(dirname("a/b/c.mp4"), "a/b");
        assert_eq!(dirname("/c.mp4"), "/");
    }
}
