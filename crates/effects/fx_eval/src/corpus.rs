//! The program corpus: every defun and def_effect, the two lookup tables of the builder.
//! Source format (MEASURED): record_extract\programs\programs.tsv, header `kind name va length text`, one row per record,
//! text on one line (no tabs, no newlines inside). `layout_effect` rows are UI layout and are counted, not parsed.

use crate::parse::{parse_item, Arena, DefunDef, EffectDef, Item};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Failure {
    pub kind: String,
    pub name: String,
    pub va: String,
    pub msg: String,
}

#[derive(Debug, Default)]
pub struct Corpus {
    pub arena: Arena,
    pub effects: Vec<EffectDef>,
    pub defuns: Vec<DefunDef>,
    /// name -> index; the last definition wins (Lua table assignment), duplicates are reported in `dup_effects`
    pub effect_idx: HashMap<String, usize>,
    pub defun_idx: HashMap<String, usize>,
    pub failures: Vec<Failure>,
    pub layout_rows: usize,
    /// LYM1: the layout_effect rows (va, text), kept for the registered-program merge (layoutmerge.rs)
    pub layout_texts: Vec<(String, String)>,
    /// the native layout rows (programs_native), as the layout reader's token tree
    pub layout_sx: Vec<Vec<crate::layoutmerge::Sx>>,
    pub dup_effects: Vec<String>,
    pub dup_defuns: Vec<String>,
}

impl Corpus {
    pub fn new() -> Corpus {
        Corpus::default()
    }

    /// Add one record text. Errors are recorded in `failures` and returned.
    pub fn add_text(&mut self, kind: &str, name: &str, va: &str, text: &str) -> Result<(), String> {
        match parse_item(&mut self.arena, text, va) {
            Ok(Item::Effect(mut e)) => {
                if e.name.is_empty() {
                    e.name = name.to_string();
                }
                let i = self.effects.len();
                if self.effect_idx.insert(e.name.clone(), i).is_some() {
                    self.dup_effects.push(e.name.clone());
                }
                self.effects.push(e);
                Ok(())
            }
            Ok(Item::Defun(d)) => {
                let i = self.defuns.len();
                if self.defun_idx.insert(d.name.clone(), i).is_some() {
                    self.dup_defuns.push(d.name.clone());
                }
                self.defuns.push(d);
                Ok(())
            }
            Err(e) => {
                let msg = e.to_string();
                self.failures.push(Failure { kind: kind.into(), name: name.into(), va: va.into(), msg: msg.clone() });
                Err(msg)
            }
        }
    }

    /// Convenience for tests and tools: add several record texts (kind inferred from the text).
    pub fn from_texts(texts: &[&str]) -> Corpus {
        let mut c = Corpus::new();
        for (i, t) in texts.iter().enumerate() {
            let kind = if t.trim_start().starts_with("(defun") { "defun" } else { "def_effect" };
            let _ = c.add_text(kind, &format!("text{}", i), "0x0", t);
        }
        c
    }

    /// Load programs.tsv text. Rows with the wrong shape are recorded as failures.
    pub fn from_programs_tsv(tsv: &str) -> Corpus {
        let mut c = Corpus::new();
        for (ln, line) in tsv.lines().enumerate() {
            if ln == 0 && line.starts_with("kind\t") {
                continue;
            }
            if line.is_empty() {
                continue;
            }
            let f: Vec<&str> = line.splitn(5, '\t').collect();
            if f.len() < 5 {
                c.failures.push(Failure { kind: "row".into(), name: String::new(), va: String::new(), msg: format!("line {}: expected 5 tab fields", ln + 1) });
                continue;
            }
            match f[0] {
                "layout_effect" => {
                    c.layout_rows += 1;
                    c.layout_texts.push((f[2].to_string(), f[4].to_string()));
                }
                "def_effect" | "defun" => {
                    let _ = c.add_text(f[0], f[1], f[2], f[4]);
                }
                other => c.failures.push(Failure { kind: other.into(), name: f[1].into(), va: f[2].into(), msg: "unknown row kind".into() }),
            }
        }
        c
    }

    pub fn find_effect(&self, name: &str, va: Option<&str>) -> Option<usize> {
        match va {
            Some(v) => self.effects.iter().position(|e| e.name == name && e.va == v),
            None => self.effect_idx.get(name).copied(),
        }
    }
}
