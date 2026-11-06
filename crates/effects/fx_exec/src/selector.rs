//! The native combo selector (slot 31, base), specified by audit_evt\COMBO_SELECT.md section 2 and
//! section 4. Exact equality, first match in table order, -1 on no match, -1 on an unbound operand, -1 on a
//! missing table (the NULL default of 28 classes). The table of a class ends at the first entry whose tag is <= 0.
//! Tables are data generated from audit_evt\COMBOS.tsv (defined in effects_rtdata: class, entries with args, tag, len).

use std::collections::HashMap;

/// One 0x68 byte table entry: `vals` are the leading ints (the rest of the 20 are -1), `tag` is, `len` is.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub vals: Vec<i32>,
    pub tag: i32,
    pub len: i32,
}

#[derive(Debug, Clone, Default)]
pub struct Tables {
    pub classes: HashMap<String, Vec<Entry>>,
}

impl Tables {
    /// Parses `class \t entry \t args \t tag \t len` rows (no header). Rows of a class are kept in file order
    /// (= entry order); the table stops at the first entry whose tag is <= 0.
    pub fn from_tsv(text: &str) -> Tables {
        let mut t = Tables::default();
        let mut ended: HashMap<String, bool> = HashMap::new();
        for line in text.lines() {
            let f: Vec<&str> = line.split('\t').collect();
            if f.len() < 5 {
                continue;
            }
            let class = f[0].to_string();
            if *ended.get(&class).unwrap_or(&false) {
                continue;
            }
            let tag: i32 = match f[3].trim().parse() {
                Ok(v) => v,
                Err(_) => continue,
            };
            let len: i32 = f[4].trim().parse().unwrap_or(-1);
            if tag <= 0 {
                ended.insert(class, true);
                continue;
            }
            let mut vals = Vec::new();
            let mut bad = false;
            if !f[2].trim().is_empty() {
                for a in f[2].split(',') {
                    match a.trim().parse::<i32>() {
                        Ok(v) => vals.push(v),
                        Err(_) => bad = true,
                    }
                }
            }
            // an entry with no ints is legal only for a zero-argument class (static2: len 0)
            if bad || (vals.is_empty() && len != 0) {
                continue;
            }
            t.classes.entry(class).or_default().push(Entry { vals, tag, len });
        }
        t
    }

    /// The selector tables of the classes the effects use, as defined in effects_rtdata (entries in table order).
    pub fn embedded() -> Tables {
        let mut t = Tables::default();
        for c in effects_rtdata::classes::COMBOS_EXEC {
            let v: Vec<Entry> = c.entries.iter().map(|e| Entry { vals: e.vals.to_vec(), tag: e.tag, len: e.len }).collect();
            t.classes.insert(c.class.to_string(), v);
        }
        t
    }

    pub fn has(&self, class: &str) -> bool {
        self.classes.contains_key(class)
    }

    /// Slot 31. `operands[i]` is the channel count of an image operand or the lane count of a constant
    /// (None = argument not bound). Returns the tag, or -1.
    pub fn select(&self, class: &str, operands: &[Option<i32>]) -> i32 {
        if operands.iter().any(|o| o.is_none()) {
            return -1;
        }
        let tab = match self.classes.get(class) {
            Some(t) => t,
            None => return -1,
        };
        for e in tab {
            let ok = operands.iter().enumerate().all(|(i, o)| {
                let want = if i < 20 { e.vals.get(i).copied().unwrap_or(-1) } else { -1 };
                want == o.unwrap_or(i32::MIN)
            });
            if ok {
                return e.tag;
            }
        }
        -1
    }

    /// Convenience for the executor: all operands bound.
    pub fn select_counts(&self, class: &str, counts: &[i32]) -> i32 {
        let v: Vec<Option<i32>> = counts.iter().map(|c| Some(*c)).collect();
        self.select(class, &v)
    }
}
