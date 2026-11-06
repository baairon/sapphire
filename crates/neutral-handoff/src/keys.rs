//! Keyframed parameter values of the frames/2 convention: `V@F;V@F;...`.
//!
//! The syntax and the interpolation follow the convention as the twixtor reference states it (the
//! "Parameter values" section of its reference and the "Key shapes" section of its timing page):
//!
//! - a key is `V@F[iS/I][oS/I][h|e]`: `F` a frame number counted from 0 (fractions allowed), a
//!   trailing `h` holds the key's value until the next key, a trailing `e` eases it, not both;
//!   `iS/I` and `oS/I` are Bezier handles into and out of the key (`S` units a second, `I` the
//!   influence in percent, 0.1 to 100, `S` at most 1e9 either way);
//! - keys are stored in ticks: an integer rate has 100 ticks a frame, a rate N/D in lowest terms
//!   D ticks a frame and N a second; `F` rounds to the nearest tick, two keys on one tick are an
//!   error, and keys may come in any order;
//! - before the first key the value is the first key's, after the last key the last one's;
//! - floats, angles and points interpolate: straight between plain keys, `u*u*(3-2*u)` between two
//!   eased keys, `u*u*(2-u)` after an eased key and `u*(1+u*(1-u))` before one; a segment with a
//!   handle at either end is the cubic Bezier solved by 64 bisection steps in f64; a float that a
//!   handle overshoots is held inside its range, an angle is not, and a point moves on the straight
//!   line between its keys;
//! - every other kind holds its value until the next key and refuses handles.

/// How a parameter's value moves between keys.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Shape {
    /// A number held inside `min` to `max` when a handle overshoots.
    Float { min: f64, max: f64 },
    /// A number that is never clamped.
    Angle,
    /// Two lanes that move on the straight line between their keys.
    Point,
    /// Held until the next key (int, popup, checkbox, color, text).
    Hold,
}

/// A Bezier handle: the speed at the key in units per tick and the influence in percent.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Handle {
    pub speed: f64,
    pub influence: f64,
}

/// One key as written and as read.
#[derive(Clone, Debug, PartialEq)]
pub struct Key {
    /// Ticks from frame 0.
    pub tick: i64,
    /// The value as written.
    pub text: String,
    /// The value's lanes (empty for a held kind whose text is not numeric).
    pub lanes: Vec<f64>,
    pub hold: bool,
    pub ease: bool,
    pub ease_in: Option<Handle>,
    pub ease_out: Option<Handle>,
}

/// One `V@F` piece split, the frame still a frame number.
#[derive(Clone, Debug, PartialEq)]
pub struct KeyText {
    pub value: String,
    pub frame: f64,
    pub hold: bool,
    pub ease: bool,
    /// Handles with their speed in units a second.
    pub ease_in: Option<Handle>,
    pub ease_out: Option<Handle>,
}

const HANDLE_SPEED: f64 = 1e9;

/// The index of the first `i` or `o` of `frame` followed by a digit or `-`.
fn handle_start(frame: &str) -> Option<usize> {
    let b = frame.as_bytes();
    (0..b.len()).find(|&i| matches!(b[i], b'i' | b'o') && matches!(b.get(i + 1), Some(b'0'..=b'9' | b'-')))
}

/// `-?[0-9]+(\.[0-9]+)?`.
fn is_number(text: &str) -> bool {
    let digits = |s: &str| !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit());
    let unsigned = text.strip_prefix('-').unwrap_or(text);
    match unsigned.split_once('.') {
        Some((int, frac)) => digits(int) && digits(frac),
        None => digits(unsigned),
    }
}

fn not_a_handle(frame: &str, text: &str) -> String {
    format!("`{}`: `{}` is not an ease handle (i or o, then SPEED/INFLUENCE, as in i0/33.3)", frame, text)
}

/// The handle at the start of `text` and the text after it.
fn handle<'a>(frame: &str, text: &'a str) -> Result<(Handle, &'a str), String> {
    let len = 1 + text[1..].bytes().take_while(|b| matches!(b, b'0'..=b'9' | b'.' | b'/' | b'-')).count();
    let (token, after) = text.split_at(len);
    let bad = || not_a_handle(frame, token);
    let (speed_text, influence_text) = token[1..].split_once('/').ok_or_else(bad)?;
    if !is_number(speed_text) || !is_number(influence_text) {
        return Err(bad());
    }
    let speed: f64 = speed_text.parse().map_err(|_| bad())?;
    let influence: f64 = influence_text.parse().map_err(|_| bad())?;
    if !(-HANDLE_SPEED..=HANDLE_SPEED).contains(&speed) {
        return Err(format!("handle speed {} is outside -{}..={}", speed_text, HANDLE_SPEED, HANDLE_SPEED));
    }
    if !(0.1..=100.0).contains(&influence) {
        return Err(format!("influence {} is outside 0.1..=100", influence_text));
    }
    Ok((Handle { speed, influence }, after))
}

/// Split one key `V@F[iS/I][oS/I][h|e]`. An error is the reason alone.
pub fn split_key(piece: &str) -> Result<KeyText, String> {
    let (value, frame) = piece.rsplit_once('@').ok_or_else(|| format!("`{}` has no @frame", piece))?;
    let frame = frame.trim();
    let not_a_frame = || format!("`{}` is not a frame number (a frame may end in h to hold or e to ease, not both)", frame);
    let start = match handle_start(frame) {
        None => {
            let (number, hold, ease) = match (frame.strip_suffix('h'), frame.strip_suffix('e')) {
                (Some(f), _) => (f, true, false),
                (_, Some(f)) => (f, false, true),
                _ => (frame, false, false),
            };
            let at: f64 = number.parse().map_err(|_| not_a_frame())?;
            if !at.is_finite() {
                return Err(not_a_frame());
            }
            return Ok(KeyText { value: value.to_string(), frame: at, hold, ease, ease_in: None, ease_out: None });
        }
        Some(s) => s,
    };
    let at: f64 = frame[..start].parse().map_err(|_| not_a_frame())?;
    if !at.is_finite() {
        return Err(not_a_frame());
    }
    let mut rest = &frame[start..];
    let mut ease_in = None;
    if rest.starts_with('i') {
        let (h, after) = handle(frame, rest)?;
        ease_in = Some(h);
        rest = after;
    }
    let mut ease_out = None;
    if rest.starts_with('o') && handle_start(rest) == Some(0) {
        let (h, after) = handle(frame, rest)?;
        ease_out = Some(h);
        rest = after;
    }
    let hold = match rest {
        "" => false,
        "h" => true,
        "e" => return Err("a key with ease handles cannot also end in e".into()),
        _ => return Err(not_a_handle(frame, rest)),
    };
    if hold && ease_out.is_some() {
        return Err("a held key has no outgoing handle".into());
    }
    Ok(KeyText { value: value.to_string(), frame: at, hold, ease: false, ease_in, ease_out })
}

/// Whether a `--param` value is keyed.
pub fn is_keyed(value: &str) -> bool {
    value.contains('@')
}

/// The pieces of a keyed value, split, with the value text of each.
pub fn split_all(value: &str) -> Result<Vec<KeyText>, String> {
    value.split(';').map(split_key).collect()
}

/// The time base of a rate `num/den` in lowest terms: (ticks a frame, ticks a second).
pub fn time_base(num: u128, den: u128) -> (i64, f64) {
    if den == 1 {
        (100, 100.0 * num as f64)
    } else {
        (den as i64, num as f64)
    }
}

/// A keyed parameter, ready to be read at any frame.
#[derive(Clone, Debug)]
pub struct Track {
    pub shape: Shape,
    pub keys: Vec<Key>,
    pub ticks_per_frame: i64,
}

impl Track {
    /// Parse `value` for a parameter of `shape` with `lanes` lanes at the time base of the rate.
    /// `lanes_of` reads one key's value text (None: not readable as numbers, which only a held kind accepts).
    pub fn parse(value: &str, shape: Shape, ticks_per_frame: i64, ticks_per_second: f64) -> Result<Track, String> {
        let mut keys = Vec::new();
        for piece in value.split(';') {
            let k = split_key(piece)?;
            if (k.ease_in.is_some() || k.ease_out.is_some()) && shape == Shape::Hold {
                return Err(format!("`{}` has ease handles, but this parameter holds its value between keys", piece.trim()));
            }
            let text = k.value.trim().to_string();
            let lanes: Vec<f64> = text.split(',').map(|t| t.trim().parse::<f64>()).collect::<Result<Vec<_>, _>>().unwrap_or_default();
            if shape != Shape::Hold && (lanes.is_empty() || lanes.iter().any(|x| !x.is_finite())) {
                return Err(format!("cannot parse `{}`", text));
            }
            let per_tick = |h: Option<Handle>| h.map(|h| Handle { speed: h.speed / ticks_per_second, influence: h.influence });
            keys.push(Key {
                tick: (k.frame * ticks_per_frame as f64).round() as i64,
                text,
                lanes,
                hold: k.hold,
                ease: k.ease,
                ease_in: per_tick(k.ease_in),
                ease_out: per_tick(k.ease_out),
            });
        }
        keys.sort_by_key(|k| k.tick);
        if keys.windows(2).any(|w| w[0].tick == w[1].tick) {
            return Err("two keyframes at the same time".into());
        }
        Ok(Track { shape, keys, ticks_per_frame })
    }

    /// The value at output frame `frame`, as `--param` text: a key's own text where the curve is a
    /// key's value, else the interpolated lanes (shortest round-trip decimal of each f64).
    pub fn text_at(&self, frame: u64) -> String {
        match self.lanes_at(frame as i64 * self.ticks_per_frame) {
            Ok(i) => self.keys[i].text.clone(),
            Err(lanes) => lanes.iter().map(|x| format!("{}", x)).collect::<Vec<_>>().join(","),
        }
    }

    /// `Ok(index)` when the value at `tick` is key `index`'s own value, else the interpolated lanes.
    pub fn lanes_at(&self, tick: i64) -> Result<usize, Vec<f64>> {
        let keys = &self.keys;
        let last = keys.len() - 1;
        if tick <= keys[0].tick {
            return Ok(0);
        }
        if tick >= keys[last].tick {
            return Ok(last);
        }
        let i = keys.partition_point(|k| k.tick <= tick) - 1;
        let (a, b) = (&keys[i], &keys[i + 1]);
        if a.hold || self.shape == Shape::Hold || tick == a.tick {
            return Ok(i);
        }
        let n = (b.tick - a.tick) as f64;
        let u = (tick - a.tick) as f64 / n;
        if a.ease_out.is_some() || b.ease_in.is_some() {
            return Err(self.bezier_at(a, b, u, n));
        }
        let f = eased_progress(u, a.ease, b.ease);
        Err(a.lanes.iter().zip(&b.lanes).map(|(x, y)| x + (y - x) * f).collect())
    }

    fn bezier_at(&self, a: &Key, b: &Key, u: f64, n: f64) -> Vec<f64> {
        match self.shape {
            Shape::Point if a.lanes.len() == 2 && b.lanes.len() == 2 => {
                let (dx, dy) = (b.lanes[0] - a.lanes[0], b.lanes[1] - a.lanes[1]);
                let l = (dx * dx + dy * dy).sqrt();
                if l == 0.0 {
                    return a.lanes.clone();
                }
                let f = bezier(u, n, 0.0, l, a, b) / l;
                vec![a.lanes[0] + dx * f, a.lanes[1] + dy * f]
            }
            Shape::Float { min, max } => a
                .lanes
                .iter()
                .zip(&b.lanes)
                .map(|(va, vb)| {
                    let y = bezier(u, n, *va, *vb - *va, a, b);
                    if y < min {
                        min
                    } else if y > max {
                        max
                    } else {
                        y
                    }
                })
                .collect(),
            _ => a.lanes.iter().zip(&b.lanes).map(|(va, vb)| bezier(u, n, *va, *vb - *va, a, b)).collect(),
        }
    }

    /// Every distinct key value text (the values a held kind takes).
    pub fn key_texts(&self) -> Vec<String> {
        self.keys.iter().map(|k| k.text.clone()).collect()
    }
}

/// The cubic Bezier of one segment `n` ticks long at progress `u`, from `va` to `va + d3`.
fn bezier(u: f64, n: f64, va: f64, d3: f64, a: &Key, b: &Key) -> f64 {
    let (x1, d1) = match a.ease_out {
        Some(h) => (h.influence / 100.0, (h.speed * (h.influence / 100.0)) * n),
        None if a.ease => (1.0 / 3.0, 0.0),
        None => (1.0 / 3.0, d3 / 3.0),
    };
    let (x2, d2) = match b.ease_in {
        Some(h) => (1.0 - h.influence / 100.0, d3 - (h.speed * (h.influence / 100.0)) * n),
        None if b.ease => (1.0 - 1.0 / 3.0, d3),
        None => (1.0 - 1.0 / 3.0, d3 - d3 / 3.0),
    };
    let (mut lo, mut hi) = (0.0f64, 1.0f64);
    for _ in 0..64 {
        let m = (lo + hi) * 0.5;
        let r = 1.0 - m;
        let x = ((3.0 * r * r * m) * x1 + (3.0 * r * m * m) * x2) + m * m * m;
        if x < u {
            lo = m;
        } else {
            hi = m;
        }
    }
    let s = (lo + hi) * 0.5;
    let r = 1.0 - s;
    va + (((3.0 * r * r * s) * d1 + (3.0 * r * s * s) * d2) + (s * s * s) * d3)
}

/// Progress along a segment reshaped by the easing of its two keys.
fn eased_progress(u: f64, ease_start: bool, ease_end: bool) -> f64 {
    match (ease_start, ease_end) {
        (false, false) => u,
        (true, true) => u * u * (3.0 - 2.0 * u),
        (true, false) => u * u * (2.0 - u),
        (false, true) => u * (1.0 + u * (1.0 - u)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn float(v: &str) -> Track {
        Track::parse(v, Shape::Float { min: 0.0, max: 1000.0 }, 100, 2400.0).unwrap()
    }

    #[test]
    fn the_four_shapes_halfway() {
        assert_eq!(float("100@0;200@10").text_at(5), "150");
        assert_eq!(float("100@0e;200@10e").text_at(5), "150");
        assert_eq!(float("100@0e;200@10").text_at(5), "137.5");
        assert_eq!(float("100@0;200@10e").text_at(5), "162.5");
        assert_eq!(float("100@0h;200@10").text_at(5), "100");
    }

    #[test]
    fn handles_follow_the_reference_case() {
        // the reference value at frame 5 of `100@0o0/100;200@10i0/100`
        assert_eq!(float("100@0o0/100;200@10i0/100").text_at(5), "149.99968569895339");
        assert_eq!(float("100@0o0/33.3;200@10i0/33.3").text_at(5).parse::<f64>().unwrap().round(), 150.0);
    }

    #[test]
    fn ends_keys_and_errors() {
        let t = float("10@4;20@2");
        assert_eq!((t.text_at(0), t.text_at(9)), ("20".to_string(), "10".to_string()));
        assert_eq!(split_key("100@0he").unwrap_err(), "`0he` is not a frame number (a frame may end in h to hold or e to ease, not both)");
        assert_eq!(split_key("50").unwrap_err(), "`50` has no @frame");
        assert_eq!(Track::parse("1@0;2@0.001", Shape::Angle, 100, 2400.0).unwrap_err(), "two keyframes at the same time");
        assert!(Track::parse("1@0i0/50;2@3", Shape::Hold, 100, 2400.0).unwrap_err().contains("holds its value"));
        assert_eq!(split_key("100@0o0/50h").unwrap_err(), "a held key has no outgoing handle");
        assert_eq!(split_key("100@0i0/0").unwrap_err(), "influence 0 is outside 0.1..=100");
    }

    #[test]
    fn time_base_and_points() {
        assert_eq!(time_base(24, 1), (100, 2400.0));
        assert_eq!(time_base(30000, 1001), (1001, 30000.0));
        let p = Track::parse("0,0@0;4,2@4", Shape::Point, 100, 2400.0).unwrap();
        assert_eq!(p.text_at(2), "2,1");
        let h = Track::parse("1@0;3@2", Shape::Hold, 100, 2400.0).unwrap();
        assert_eq!(h.text_at(1), "1");
    }
}
