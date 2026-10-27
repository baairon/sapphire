//! [`Out`]: a small JSON value that prints the way Python's `json.dumps` does.
//!
//! `tools/apply.py --json` prints `json.dumps(result, indent=2)`, and its error line uses the
//! default separators. This type reproduces both layouts byte for byte for the values the
//! tools print: keys keep their insertion order, every character outside printable ASCII is
//! written as a `\uXXXX` escape (a surrogate pair above U+FFFF), and a float is written with the
//! shortest text that reads back to the same value, with `.0` for whole numbers.

/// A JSON value for output.
#[derive(Clone, Debug, PartialEq)]
pub enum Out {
    /// `null`
    Null,
    /// `true` or `false`
    Bool(bool),
    /// An integer.
    Int(i64),
    /// A float (never NaN or infinite in this crate's output).
    Float(f64),
    /// A string.
    Str(String),
    /// An array.
    List(Vec<Out>),
    /// An object with its members in insertion order.
    Map(Vec<(String, Out)>),
}

impl Out {
    /// A string value.
    pub fn text(value: impl Into<String>) -> Out {
        Out::Str(value.into())
    }

    /// A string value, or `null`.
    pub fn opt_text(value: Option<impl Into<String>>) -> Out {
        match value {
            Some(v) => Out::Str(v.into()),
            None => Out::Null,
        }
    }

    /// An integer value, or `null`.
    pub fn opt_int(value: Option<i64>) -> Out {
        value.map(Out::Int).unwrap_or(Out::Null)
    }

    /// A float value, or `null`.
    pub fn opt_float(value: Option<f64>) -> Out {
        value.map(Out::Float).unwrap_or(Out::Null)
    }

    /// An object from `(key, value)` pairs.
    pub fn map<K: Into<String>>(members: Vec<(K, Out)>) -> Out {
        Out::Map(members.into_iter().map(|(k, v)| (k.into(), v)).collect())
    }

    /// Python's `json.dumps(value)`: members separated by `, ` and `: `, on one line.
    pub fn dumps(&self) -> String {
        let mut out = String::new();
        self.write(&mut out, None, 0);
        out
    }

    /// Python's `json.dumps(value, indent=n)`.
    pub fn dumps_indent(&self, indent: usize) -> String {
        let mut out = String::new();
        self.write(&mut out, Some(indent), 0);
        out
    }

    /// The member `key` of an object.
    pub fn get(&self, key: &str) -> Option<&Out> {
        match self {
            Out::Map(members) => members.iter().find(|(k, _)| k == key).map(|(_, v)| v),
            _ => None,
        }
    }

    fn write(&self, out: &mut String, indent: Option<usize>, level: usize) {
        match self {
            Out::Null => out.push_str("null"),
            Out::Bool(true) => out.push_str("true"),
            Out::Bool(false) => out.push_str("false"),
            Out::Int(n) => out.push_str(&n.to_string()),
            Out::Float(x) => out.push_str(&float_repr(*x)),
            Out::Str(s) => write_string(out, s),
            Out::List(items) => {
                if items.is_empty() {
                    out.push_str("[]");
                    return;
                }
                out.push('[');
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        out.push_str(if indent.is_some() { "," } else { ", " });
                    }
                    newline(out, indent, level + 1);
                    item.write(out, indent, level + 1);
                }
                newline(out, indent, level);
                out.push(']');
            }
            Out::Map(members) => {
                if members.is_empty() {
                    out.push_str("{}");
                    return;
                }
                out.push('{');
                for (i, (key, value)) in members.iter().enumerate() {
                    if i > 0 {
                        out.push_str(if indent.is_some() { "," } else { ", " });
                    }
                    newline(out, indent, level + 1);
                    write_string(out, key);
                    out.push_str(": ");
                    value.write(out, indent, level + 1);
                }
                newline(out, indent, level);
                out.push('}');
            }
        }
    }
}

fn newline(out: &mut String, indent: Option<usize>, level: usize) {
    if let Some(width) = indent {
        out.push('\n');
        for _ in 0..width * level {
            out.push(' ');
        }
    }
}

fn float_repr(x: f64) -> String {
    if x.is_nan() {
        "NaN".to_string()
    } else if x.is_infinite() {
        if x > 0.0 { "Infinity" } else { "-Infinity" }.to_string()
    } else {
        format!("{:?}", x)
    }
}

fn write_string(out: &mut String, s: &str) {
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{8}' => out.push_str("\\b"),
            '\u{c}' => out.push_str("\\f"),
            ' '..='~' => out.push(c),
            _ => {
                let mut units = [0u16; 2];
                for unit in c.encode_utf16(&mut units) {
                    out.push_str(&format!("\\u{:04x}", unit));
                }
            }
        }
    }
    out.push('"');
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compact_layout_uses_the_default_separators() {
        let v = Out::map(vec![("ok", Out::Bool(false)), ("error", Out::text("x")), ("n", Out::Int(-3))]);
        assert_eq!(v.dumps(), r#"{"ok": false, "error": "x", "n": -3}"#);
        assert_eq!(Out::List(vec![]).dumps(), "[]");
        assert_eq!(Out::Map(vec![]).dumps(), "{}");
    }

    #[test]
    fn indented_layout_matches_python() {
        let v = Out::map(vec![
            ("a", Out::List(vec![Out::Int(1), Out::Int(2)])),
            ("b", Out::map(vec![("c", Out::Null)])),
            ("d", Out::List(vec![])),
        ]);
        let want = "{\n  \"a\": [\n    1,\n    2\n  ],\n  \"b\": {\n    \"c\": null\n  },\n  \"d\": []\n}";
        assert_eq!(v.dumps_indent(2), want);
    }

    #[test]
    fn strings_are_escaped_like_ensure_ascii() {
        assert_eq!(Out::text("a\"b\\c\n\t").dumps(), r#""a\"b\\c\n\t""#);
        assert_eq!(Out::text("\u{e9}").dumps(), r#""\u00e9""#);
        assert_eq!(Out::text("\u{1f600}").dumps(), r#""\ud83d\ude00""#);
        assert_eq!(Out::text("\u{7f}\u{1}").dumps(), r#""\u007f\u0001""#);
        assert_eq!(Out::text("C:\\data\\x").dumps(), r#""C:\\data\\x""#);
    }

    #[test]
    fn floats_print_like_repr() {
        assert_eq!(Out::Float(24.0).dumps(), "24.0");
        assert_eq!(Out::Float(0.5).dumps(), "0.5");
        assert_eq!(Out::Float(30000.0 / 1001.0).dumps(), "29.97002997002997");
    }
}
