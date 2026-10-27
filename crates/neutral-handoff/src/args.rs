//! A small command line reader shared by the wrapper and the `stage` subcommand.
//!
//! It accepts `--flag value`, `--flag=value`, `-o value`, `-ovalue`, `--` and repeated flags,
//! and reports problems as `usage` errors in the words of `argparse`. Unique prefixes of long
//! flags (`--dry` for `--dry-run`) are not accepted.

use crate::error::{Error, Result};

/// One token of the command line.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token {
    /// A flag, without its value: `--param`, `-o`.
    Flag(String),
    /// A flag with the value written after `=` (or glued to a short flag).
    FlagValue(String, String),
    /// A plain argument.
    Positional(String),
}

fn looks_like_number(text: &str) -> bool {
    text.parse::<f64>().is_ok()
}

/// Split the command line into tokens.
pub fn tokenize(args: &[String]) -> Vec<Token> {
    let mut out = Vec::new();
    let mut rest_is_positional = false;
    for arg in args {
        if rest_is_positional {
            out.push(Token::Positional(arg.clone()));
        } else if arg == "--" {
            rest_is_positional = true;
        } else if let Some(long) = arg.strip_prefix("--") {
            match long.split_once('=') {
                Some((name, value)) => out.push(Token::FlagValue(format!("--{}", name), value.to_string())),
                None => out.push(Token::Flag(arg.clone())),
            }
        } else if arg.len() > 1 && arg.starts_with('-') && !looks_like_number(arg) {
            if arg.len() > 2 {
                out.push(Token::FlagValue(arg[..2].to_string(), arg[2..].to_string()));
            } else {
                out.push(Token::Flag(arg.clone()));
            }
        } else {
            out.push(Token::Positional(arg.clone()));
        }
    }
    out
}

/// Walks the tokens, handing out values to flags that take one.
pub struct Cursor {
    tokens: Vec<Token>,
    at: usize,
}

impl Cursor {
    /// A cursor over `args`.
    pub fn new(args: &[String]) -> Cursor {
        Cursor { tokens: tokenize(args), at: 0 }
    }

    /// The next token, or `None` at the end.
    pub fn next_token(&mut self) -> Option<Token> {
        let t = self.tokens.get(self.at).cloned();
        if t.is_some() {
            self.at += 1;
        }
        t
    }

    /// The value of `flag`: the text after `=` if it had one, else the next token.
    pub fn value(&mut self, flag: &str, inline: Option<String>) -> Result<String> {
        if let Some(v) = inline {
            return Ok(v);
        }
        match self.tokens.get(self.at) {
            Some(Token::Positional(v)) => {
                let v = v.clone();
                self.at += 1;
                Ok(v)
            }
            _ => Err(Error::usage(format!("argument {}: expected one argument", flag))),
        }
    }
}

/// `argument FLAG: invalid VALUE`-style error.
pub fn bad_value(flag: &str, message: &str) -> Error {
    Error::usage(format!("argument {}: {}", flag, message))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn toks(args: &[&str]) -> Vec<Token> {
        tokenize(&args.iter().map(|s| s.to_string()).collect::<Vec<_>>())
    }

    #[test]
    fn long_short_and_inline_forms() {
        assert_eq!(
            toks(&["in.mp4", "--param", "a=1", "--depth=8", "-o", "out.mp4", "-oother.mp4", "-h"]),
            vec![
                Token::Positional("in.mp4".into()),
                Token::Flag("--param".into()),
                Token::Positional("a=1".into()),
                Token::FlagValue("--depth".into(), "8".into()),
                Token::Flag("-o".into()),
                Token::Positional("out.mp4".into()),
                Token::FlagValue("-o".into(), "other.mp4".into()),
                Token::Flag("-h".into()),
            ]
        );
    }

    #[test]
    fn double_dash_ends_the_flags() {
        assert_eq!(toks(&["--", "--odd"]), vec![Token::Positional("--odd".into())]);
    }

    #[test]
    fn a_lone_dash_and_negative_numbers_are_positional() {
        assert_eq!(toks(&["-", "-5", "-0.5"]), vec![Token::Positional("-".into()), Token::Positional("-5".into()), Token::Positional("-0.5".into())]);
    }

    #[test]
    fn a_value_with_equals_keeps_the_rest() {
        assert_eq!(toks(&["--param=speed_percent=40"]), vec![Token::FlagValue("--param".into(), "speed_percent=40".into())]);
    }

    #[test]
    fn cursor_hands_out_values() {
        let mut c = Cursor::new(&["--param".to_string(), "a=1".to_string(), "--json".to_string()]);
        assert_eq!(c.next_token(), Some(Token::Flag("--param".into())));
        assert_eq!(c.value("--param", None).unwrap(), "a=1");
        assert_eq!(c.next_token(), Some(Token::Flag("--json".into())));
        let err = c.value("--json", None).unwrap_err();
        assert_eq!(err.message, "argument --json: expected one argument");
        assert_eq!(c.next_token(), None);
        assert_eq!(Cursor::new(&[]).value("--x", Some("v".into())).unwrap(), "v");
    }

    #[test]
    fn a_flag_cannot_take_another_flag_as_its_value() {
        let mut c = Cursor::new(&["--output".to_string(), "--force".to_string()]);
        assert_eq!(c.next_token(), Some(Token::Flag("--output".into())));
        assert!(c.value("--output", None).is_err());
    }
}
