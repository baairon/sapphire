//! A small std-only JSON parser, enough for ffprobe's output.
//!
//! Structure is validated strictly (no trailing commas, no leading zeros, no
//! comments, no stray text after the value). String contents are lenient in one
//! way: bytes that are not valid UTF-8 and unpaired surrogate escapes become
//! U+FFFD, because file metadata printed by ffprobe is not always clean.

use std::fmt;

use crate::error::{Error, Result};

/// Deepest array/object nesting accepted.
pub const MAX_DEPTH: usize = 128;

/// A parsed JSON value.
#[derive(Clone, Debug, PartialEq)]
pub enum Json {
    /// `null`
    Null,
    /// `true` or `false`
    Bool(bool),
    /// A number written without fraction or exponent that fits `i64`.
    Int(i64),
    /// Any other number.
    Float(f64),
    /// A string with escapes resolved.
    Str(String),
    /// An array.
    Array(Vec<Json>),
    /// An object, members in file order (duplicate keys are kept).
    Object(Vec<(String, Json)>),
}

impl Json {
    /// Parse a complete JSON document from text.
    pub fn parse(text: &str) -> Result<Json> {
        Json::parse_bytes(text.as_bytes())
    }

    /// Parse a complete JSON document from bytes.
    pub fn parse_bytes(bytes: &[u8]) -> Result<Json> {
        let mut p = Parser { s: bytes, i: 0 };
        p.skip_ws();
        let v = p.value(0)?;
        p.skip_ws();
        if p.i != p.s.len() {
            return Err(p.err("unexpected data after the JSON value"));
        }
        Ok(v)
    }

    /// First member named `key` when this is an object.
    pub fn get(&self, key: &str) -> Option<&Json> {
        match self {
            Json::Object(members) => members.iter().find(|(k, _)| k == key).map(|(_, v)| v),
            _ => None,
        }
    }

    /// Element `index` when this is an array.
    pub fn index(&self, index: usize) -> Option<&Json> {
        match self {
            Json::Array(items) => items.get(index),
            _ => None,
        }
    }

    /// The string, when this is one.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Json::Str(s) => Some(s),
            _ => None,
        }
    }

    /// The boolean, when this is one.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Json::Bool(b) => Some(*b),
            _ => None,
        }
    }

    /// The number as `i64` when it is an integer (or a float without fraction in range).
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Json::Int(i) => Some(*i),
            Json::Float(f) if f.fract() == 0.0 && *f >= -9.223_372_036_854_776e18 && *f < 9.223_372_036_854_776e18 => {
                Some(*f as i64)
            }
            _ => None,
        }
    }

    /// The number as `u64` when it is a non-negative integer.
    pub fn as_u64(&self) -> Option<u64> {
        self.as_i64().and_then(|i| u64::try_from(i).ok())
    }

    /// The number as `f64`.
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Json::Int(i) => Some(*i as f64),
            Json::Float(f) => Some(*f),
            _ => None,
        }
    }

    /// The elements, when this is an array.
    pub fn as_array(&self) -> Option<&[Json]> {
        match self {
            Json::Array(items) => Some(items),
            _ => None,
        }
    }

    /// The members, when this is an object.
    pub fn as_object(&self) -> Option<&[(String, Json)]> {
        match self {
            Json::Object(members) => Some(members),
            _ => None,
        }
    }

    /// True for `null`.
    pub fn is_null(&self) -> bool {
        matches!(self, Json::Null)
    }
}

fn write_escaped(f: &mut fmt::Formatter<'_>, s: &str) -> fmt::Result {
    f.write_str("\"")?;
    for c in s.chars() {
        match c {
            '"' => f.write_str("\\\"")?,
            '\\' => f.write_str("\\\\")?,
            '\n' => f.write_str("\\n")?,
            '\r' => f.write_str("\\r")?,
            '\t' => f.write_str("\\t")?,
            '\u{8}' => f.write_str("\\b")?,
            '\u{c}' => f.write_str("\\f")?,
            c if (c as u32) < 0x20 => write!(f, "\\u{:04x}", c as u32)?,
            c => write!(f, "{}", c)?,
        }
    }
    f.write_str("\"")
}

impl fmt::Display for Json {
    /// Compact JSON text. Non-finite floats are written as `null`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Json::Null => f.write_str("null"),
            Json::Bool(b) => write!(f, "{}", b),
            Json::Int(i) => write!(f, "{}", i),
            Json::Float(x) if x.is_finite() => {
                let s = format!("{:?}", x);
                f.write_str(&s)
            }
            Json::Float(_) => f.write_str("null"),
            Json::Str(s) => write_escaped(f, s),
            Json::Array(items) => {
                f.write_str("[")?;
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        f.write_str(",")?;
                    }
                    write!(f, "{}", item)?;
                }
                f.write_str("]")
            }
            Json::Object(members) => {
                f.write_str("{")?;
                for (i, (k, v)) in members.iter().enumerate() {
                    if i > 0 {
                        f.write_str(",")?;
                    }
                    write_escaped(f, k)?;
                    f.write_str(":")?;
                    write!(f, "{}", v)?;
                }
                f.write_str("}")
            }
        }
    }
}

struct Parser<'a> {
    s: &'a [u8],
    i: usize,
}

impl<'a> Parser<'a> {
    fn err(&self, message: &str) -> Error {
        Error::Json { offset: self.i, message: message.to_string() }
    }

    fn skip_ws(&mut self) {
        while self.i < self.s.len() && matches!(self.s[self.i], b' ' | b'\t' | b'\n' | b'\r') {
            self.i += 1;
        }
    }

    fn peek(&self) -> Option<u8> {
        self.s.get(self.i).copied()
    }

    fn literal(&mut self, word: &[u8], value: Json) -> Result<Json> {
        if self.s[self.i..].starts_with(word) {
            self.i += word.len();
            Ok(value)
        } else {
            Err(self.err("invalid literal"))
        }
    }

    fn value(&mut self, depth: usize) -> Result<Json> {
        match self.peek() {
            None => Err(self.err("unexpected end of input")),
            Some(b'{') => self.object(depth),
            Some(b'[') => self.array(depth),
            Some(b'"') => Ok(Json::Str(self.string()?)),
            Some(b't') => self.literal(b"true", Json::Bool(true)),
            Some(b'f') => self.literal(b"false", Json::Bool(false)),
            Some(b'n') => self.literal(b"null", Json::Null),
            Some(b'-') | Some(b'0'..=b'9') => self.number(),
            Some(_) => Err(self.err("unexpected character")),
        }
    }

    fn object(&mut self, depth: usize) -> Result<Json> {
        if depth >= MAX_DEPTH {
            return Err(self.err("nesting is too deep"));
        }
        self.i += 1; // {
        let mut members = Vec::new();
        self.skip_ws();
        if self.peek() == Some(b'}') {
            self.i += 1;
            return Ok(Json::Object(members));
        }
        loop {
            self.skip_ws();
            if self.peek() != Some(b'"') {
                return Err(self.err("expected a string key"));
            }
            let key = self.string()?;
            self.skip_ws();
            if self.peek() != Some(b':') {
                return Err(self.err("expected ':' after the key"));
            }
            self.i += 1;
            self.skip_ws();
            let v = self.value(depth + 1)?;
            members.push((key, v));
            self.skip_ws();
            match self.peek() {
                Some(b',') => self.i += 1,
                Some(b'}') => {
                    self.i += 1;
                    return Ok(Json::Object(members));
                }
                _ => return Err(self.err("expected ',' or '}'")),
            }
        }
    }

    fn array(&mut self, depth: usize) -> Result<Json> {
        if depth >= MAX_DEPTH {
            return Err(self.err("nesting is too deep"));
        }
        self.i += 1; // [
        let mut items = Vec::new();
        self.skip_ws();
        if self.peek() == Some(b']') {
            self.i += 1;
            return Ok(Json::Array(items));
        }
        loop {
            self.skip_ws();
            items.push(self.value(depth + 1)?);
            self.skip_ws();
            match self.peek() {
                Some(b',') => self.i += 1,
                Some(b']') => {
                    self.i += 1;
                    return Ok(Json::Array(items));
                }
                _ => return Err(self.err("expected ',' or ']'")),
            }
        }
    }

    fn hex4(&mut self) -> Result<u32> {
        if self.i + 4 > self.s.len() {
            return Err(self.err("truncated \\u escape"));
        }
        let mut v = 0u32;
        for k in 0..4 {
            let d = (self.s[self.i + k] as char)
                .to_digit(16)
                .ok_or_else(|| self.err("invalid hex digit in \\u escape"))?;
            v = v * 16 + d;
        }
        self.i += 4;
        Ok(v)
    }

    fn string(&mut self) -> Result<String> {
        self.i += 1; // opening quote
        let mut out: Vec<u8> = Vec::new();
        loop {
            let b = match self.peek() {
                None => return Err(self.err("unterminated string")),
                Some(b) => b,
            };
            match b {
                b'"' => {
                    self.i += 1;
                    return Ok(String::from_utf8_lossy(&out).into_owned());
                }
                b'\\' => {
                    self.i += 1;
                    let e = self.peek().ok_or_else(|| self.err("unterminated escape"))?;
                    self.i += 1;
                    match e {
                        b'"' => out.push(b'"'),
                        b'\\' => out.push(b'\\'),
                        b'/' => out.push(b'/'),
                        b'b' => out.push(8),
                        b'f' => out.push(12),
                        b'n' => out.push(b'\n'),
                        b'r' => out.push(b'\r'),
                        b't' => out.push(b'\t'),
                        b'u' => {
                            let first = self.hex4()?;
                            let ch = if (0xD800..0xDC00).contains(&first) {
                                if self.s[self.i..].starts_with(b"\\u") {
                                    let save = self.i;
                                    self.i += 2;
                                    let second = self.hex4()?;
                                    if (0xDC00..0xE000).contains(&second) {
                                        let cp = 0x10000 + ((first - 0xD800) << 10) + (second - 0xDC00);
                                        char::from_u32(cp).unwrap_or('\u{FFFD}')
                                    } else {
                                        self.i = save;
                                        '\u{FFFD}'
                                    }
                                } else {
                                    '\u{FFFD}'
                                }
                            } else if (0xDC00..0xE000).contains(&first) {
                                '\u{FFFD}'
                            } else {
                                char::from_u32(first).unwrap_or('\u{FFFD}')
                            };
                            let mut buf = [0u8; 4];
                            out.extend_from_slice(ch.encode_utf8(&mut buf).as_bytes());
                        }
                        _ => {
                            self.i -= 1;
                            return Err(self.err("invalid escape"));
                        }
                    }
                }
                0..=0x1f => return Err(self.err("control character in string")),
                _ => {
                    out.push(b);
                    self.i += 1;
                }
            }
        }
    }

    fn number(&mut self) -> Result<Json> {
        let start = self.i;
        if self.peek() == Some(b'-') {
            self.i += 1;
        }
        match self.peek() {
            Some(b'0') => {
                self.i += 1;
                if matches!(self.peek(), Some(b'0'..=b'9')) {
                    return Err(self.err("leading zero in number"));
                }
            }
            Some(b'1'..=b'9') => {
                while matches!(self.peek(), Some(b'0'..=b'9')) {
                    self.i += 1;
                }
            }
            _ => return Err(self.err("invalid number")),
        }
        let mut is_float = false;
        if self.peek() == Some(b'.') {
            is_float = true;
            self.i += 1;
            if !matches!(self.peek(), Some(b'0'..=b'9')) {
                return Err(self.err("digits expected after '.'"));
            }
            while matches!(self.peek(), Some(b'0'..=b'9')) {
                self.i += 1;
            }
        }
        if matches!(self.peek(), Some(b'e') | Some(b'E')) {
            is_float = true;
            self.i += 1;
            if matches!(self.peek(), Some(b'+') | Some(b'-')) {
                self.i += 1;
            }
            if !matches!(self.peek(), Some(b'0'..=b'9')) {
                return Err(self.err("digits expected in exponent"));
            }
            while matches!(self.peek(), Some(b'0'..=b'9')) {
                self.i += 1;
            }
        }
        let text = std::str::from_utf8(&self.s[start..self.i]).map_err(|_| self.err("invalid number"))?;
        if !is_float {
            if let Ok(i) = text.parse::<i64>() {
                return Ok(Json::Int(i));
            }
        }
        text.parse::<f64>().map(Json::Float).map_err(|_| self.err("invalid number"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ok(text: &str) -> Json {
        Json::parse(text).unwrap_or_else(|e| panic!("{:?} should parse: {}", text, e))
    }

    fn bad(text: &str) {
        assert!(Json::parse(text).is_err(), "{:?} should be rejected", text);
    }

    #[test]
    fn scalars() {
        assert_eq!(ok("null"), Json::Null);
        assert_eq!(ok(" true "), Json::Bool(true));
        assert_eq!(ok("false"), Json::Bool(false));
        assert_eq!(ok("0"), Json::Int(0));
        assert_eq!(ok("-17"), Json::Int(-17));
        assert_eq!(ok("1.5"), Json::Float(1.5));
        assert_eq!(ok("-2.5e3"), Json::Float(-2500.0));
        assert_eq!(ok("1E2"), Json::Float(100.0));
        assert_eq!(ok("9223372036854775807"), Json::Int(i64::MAX));
        assert!(matches!(ok("9223372036854775808"), Json::Float(_)));
    }

    #[test]
    fn strings_and_escapes() {
        assert_eq!(ok(r#""abc""#), Json::Str("abc".into()));
        assert_eq!(ok(r#""a\"b\\c\/d""#), Json::Str("a\"b\\c/d".into()));
        assert_eq!(ok(r#""\b\f\n\r\t""#), Json::Str("\u{8}\u{c}\n\r\t".into()));
        assert_eq!(ok(r#""\u0041\u00e9""#), Json::Str("A\u{e9}".into()));
        assert_eq!(ok(r#""\ud83d\ude00""#), Json::Str("\u{1F600}".into()));
        assert_eq!(ok(r#""\ud83d""#), Json::Str("\u{FFFD}".into()));
        assert_eq!(ok(r#""\ude00x""#), Json::Str("\u{FFFD}x".into()));
        assert_eq!(ok(r#""\ud83d\u0041""#), Json::Str("\u{FFFD}A".into()));
        assert_eq!(ok("\"caf\u{e9} \u{1F600}\""), Json::Str("caf\u{e9} \u{1F600}".into()));
    }

    #[test]
    fn invalid_utf8_in_strings_is_replaced() {
        let v = Json::parse_bytes(b"\"a\xffb\"").unwrap();
        assert_eq!(v, Json::Str("a\u{FFFD}b".into()));
    }

    #[test]
    fn containers() {
        let v = ok(r#" { "a": [1, 2.5, "x", null, true], "b": {"c": {}}, "d": [] } "#);
        assert_eq!(v.get("a").unwrap().as_array().unwrap().len(), 5);
        assert_eq!(v.get("a").unwrap().index(1), Some(&Json::Float(2.5)));
        assert_eq!(v.get("b").unwrap().get("c"), Some(&Json::Object(vec![])));
        assert_eq!(v.get("d"), Some(&Json::Array(vec![])));
        assert_eq!(v.get("missing"), None);
        assert_eq!(v.get("a").unwrap().index(2).unwrap().as_str(), Some("x"));
        assert!(v.get("a").unwrap().index(3).unwrap().is_null());
    }

    #[test]
    fn duplicate_keys_keep_order_and_get_returns_first() {
        let v = ok(r#"{"k":1,"k":2}"#);
        assert_eq!(v.as_object().unwrap().len(), 2);
        assert_eq!(v.get("k").unwrap().as_i64(), Some(1));
    }

    #[test]
    fn numeric_accessors() {
        assert_eq!(Json::Int(5).as_u64(), Some(5));
        assert_eq!(Json::Int(-5).as_u64(), None);
        assert_eq!(Json::Float(3.0).as_i64(), Some(3));
        assert_eq!(Json::Float(3.5).as_i64(), None);
        assert_eq!(Json::Int(2).as_f64(), Some(2.0));
        assert_eq!(Json::Str("2".into()).as_f64(), None);
    }

    #[test]
    fn rejects_malformed_documents() {
        for text in [
            "", " ", "{", "}", "[1,]", "[,1]", "{\"a\":}", "{\"a\" 1}", "{a:1}", "{\"a\":1,}", "[1 2]",
            "01", "-", "1.", ".5", "1e", "+1", "tru", "nul", "'a'", "\"abc", "\"\\x\"", "\"\\u12\"",
            "\"\\u12g4\"", "1 2", "[] x", "{\"a\":1} {}", "\"a\nb\"", "[1,2", "NaN", "Infinity",
        ] {
            bad(text);
        }
    }

    #[test]
    fn error_offsets_point_at_the_problem() {
        match Json::parse("[1, 2, x]") {
            Err(Error::Json { offset, .. }) => assert_eq!(offset, 7),
            other => panic!("{:?}", other),
        }
    }

    #[test]
    fn depth_is_limited() {
        let deep_ok = format!("{}{}", "[".repeat(MAX_DEPTH), "]".repeat(MAX_DEPTH));
        assert!(Json::parse(&deep_ok).is_ok());
        let too_deep = format!("{}{}", "[".repeat(MAX_DEPTH + 1), "]".repeat(MAX_DEPTH + 1));
        assert!(Json::parse(&too_deep).is_err());
        let huge = format!("{}{}", "[".repeat(100_000), "]".repeat(100_000));
        assert!(Json::parse(&huge).is_err(), "must fail cleanly instead of overflowing the stack");
    }

    #[test]
    fn display_round_trips() {
        let text = r#"{"s":"a\"b\\c\n\u0001","n":[1,-2,3.25,1e300],"t":true,"f":false,"z":null,"o":{"k":[]}}"#;
        let v = ok(text);
        let again = ok(&v.to_string());
        assert_eq!(v, again);
    }

    #[test]
    fn ffprobe_shaped_document() {
        let doc = r#"{
            "streams": [
                {"index": 0, "codec_name": "ffv1", "codec_type": "video", "width": 64, "height": 48,
                 "pix_fmt": "yuv420p", "r_frame_rate": "10/1", "avg_frame_rate": "10/1",
                 "duration": "1.000000", "nb_frames": "10"},
                {"index": 1, "codec_name": "pcm_s16le", "codec_type": "audio", "sample_rate": "44100", "channels": 1}
            ],
            "format": {"format_name": "matroska,webm", "duration": "1.000000"}
        }"#;
        let v = ok(doc);
        assert_eq!(v.get("streams").unwrap().as_array().unwrap().len(), 2);
        assert_eq!(v.get("format").unwrap().get("format_name").unwrap().as_str(), Some("matroska,webm"));
    }
}
