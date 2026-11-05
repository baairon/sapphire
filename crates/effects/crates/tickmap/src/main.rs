//! tickmap_case: reads a case TSV (written by t2/gen_cases.py), runs `tickmap::map_param` with the same
//! synthetic host as the emulator stubs and prints one result line per case:
//! id <TAB> events joined by '|' <TAB> ok | err:<Name> <TAB> first n+2 destination words (hex, comma separated)
use std::collections::HashMap;
use std::io::{BufRead, Write};
use tickmap::*;

struct Syn {
    hv: HashMap<Option<i32>, (i64, i64, i64)>,
    nt: Vec<Option<u32>>,
    ev: Vec<String>,
    nt_n: usize,
}

impl Host for Syn {
    fn checkout(&mut self, r: u64, i: i32, t: i32, s: i32, sc: i32) -> (i32, i32) {
        self.ev.push(format!("CO:{},{},{},{},{}", r, i, t, s, sc));
        let (b, sl, f) = *self.hv.get(&Some(i)).or_else(|| self.hv.get(&None)).unwrap_or(&(0, 0, 0));
        let raw = (b.wrapping_add(sl.wrapping_mul(t as i64))) as u32 as i32;
        if f != 0 { (f as u32 as i32, 0) } else { (0, raw) }
    }
    fn checkin(&mut self, r: u64) {
        self.ev.push(format!("CI:{}", r));
    }
    fn fetch_no_time(&mut self, a: i32, i: i32, c: u64, out: &mut [u8; 41]) {
        self.ev.push(format!("NT:{},{},{}", a, i, c));
        let w = self.nt.get(self.nt_n).copied().flatten();
        self.nt_n += 1;
        if let Some(w) = w {
            out[4..8].copy_from_slice(&w.to_le_bytes());
        }
    }
}

fn hx(s: &str) -> u32 {
    u32::from_str_radix(s, 16).unwrap()
}

fn main() {
    let path = std::env::args().nth(1).expect("cases tsv");
    let f = std::io::BufReader::new(std::fs::File::open(path).unwrap());
    let so = std::io::stdout();
    let mut so = so.lock();
    for line in f.lines() {
        let line = line.unwrap();
        if line.is_empty() || line.starts_with('#') || line.starts_with("id\t") {
            continue;
        }
        let c: Vec<&str> = line.split('\t').collect();
        let id = c[0];
        let t = f32::from_bits(hx(c[2]));
        let pinfo = Pinfo { time_frames: f32::from_bits(hx(c[3])), selector: c[4].parse().unwrap(), unit_x3: 1.0, unit_d4: 0.0, unit_d5: 0.0 };
        let m: i32 = c[5].parse().unwrap();
        let f4: i32 = c[6].parse().unwrap();
        let gid: i32 = c[7].parse().unwrap();
        let scale: i32 = c[8].parse().unwrap();
        let eref: u64 = c[9].parse().unwrap();
        let rec = ParamRec {
            ptype: c[10].parse().unwrap(),
            nlanes: c[11].parse().unwrap(),
            kind_byte: c[12].parse().unwrap(),
            subtype_byte: c[13].parse().unwrap(),
            flags_a_byte: c[14].parse().unwrap(),
            flags_b_byte: c[15].parse().unwrap(),
            index: c[16].parse().unwrap(),
            rec_unit_value: 1.0,
        };
        let hr = HostRec { effect_ref: eref, time_mult: m, time_scale: scale, double_flag: f4, size_x: c[17].parse().unwrap(), size_y: c[18].parse().unwrap() };
        let nto = NoTimeObj { a: c[19].parse().unwrap(), c: c[20].parse().unwrap() };
        let mut hv = HashMap::new();
        for it in c[21].split(';').filter(|x| !x.is_empty()) {
            let p: Vec<&str> = it.split(':').collect();
            let key = if p[0] == "*" { None } else { Some(p[0].parse::<i32>().unwrap()) };
            hv.insert(key, (p[1].parse::<i64>().unwrap(), p[2].parse::<i64>().unwrap(), p[3].parse::<i64>().unwrap()));
        }
        let nt: Vec<Option<u32>> = c[22].split('|').filter(|x| !x.is_empty()).map(|x| if x == "-" { None } else { Some(hx(x)) }).collect();
        let nohost = c.get(23).map(|x| *x == "1").unwrap_or(false);
        let mut h = Syn { hv, nt, ev: vec![], nt_n: 0 };
        let mut dest = [f32::from_bits(0xa5a5a5a5); 256];
        let r = map_param(&pinfo, if nohost { None } else { Some(&hr) }, Some(&nto), gid, &rec, t, &mut h, &mut dest, None);
        let n = (rec.nlanes as usize + 2).min(256);
        let words: Vec<String> = dest[..n].iter().map(|x| format!("{:08x}", x.to_bits())).collect();
        let res = match r {
            Ok(()) => "ok".to_string(),
            Err(e) => format!("err:{:?}", e),
        };
        writeln!(so, "{}\t{}\t{}\t{}", id, h.ev.join("|"), res, words.join(",")).unwrap();
    }
}
