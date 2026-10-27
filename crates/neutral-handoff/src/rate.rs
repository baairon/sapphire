//! [`Rate`]: an exact, positive frame rate as a reduced fraction.
//!
//! `tools/apply.py` keeps the input's rate as a Python `Fraction` so that `vfx` and the encoder
//! get `30000/1001` and never `29.97`. This type does the same with integers: it parses what
//! `Fraction(str)` accepts for a positive value (`n/d`, an integer, a decimal, an exponent),
//! reduces it, and prints it as `N/D`.

use std::fmt;

/// A positive rational number, always in lowest terms.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Rate {
    num: u128,
    den: u128,
}

fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

fn digits(text: &str) -> Option<u128> {
    if text.is_empty() || text.len() > 30 || !text.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    text.parse().ok()
}

impl Rate {
    /// `num / den` reduced, or `None` when either is zero.
    pub fn new(num: u128, den: u128) -> Option<Rate> {
        if num == 0 || den == 0 {
            return None;
        }
        let g = gcd(num, den);
        Some(Rate { num: num / g, den: den / g })
    }

    /// The numerator of the reduced fraction.
    pub fn num(&self) -> u128 {
        self.num
    }

    /// The denominator of the reduced fraction.
    pub fn den(&self) -> u128 {
        self.den
    }

    /// Read `text` the way `Fraction(str)` does, keeping only positive, non-zero values.
    ///
    /// Accepts `30000/1001`, `25`, `29.97`, `1e3` and surrounding blanks. `0/0`, `0`, negative
    /// numbers and anything else give `None`, like `apply.py`'s `_fraction`.
    pub fn parse(text: &str) -> Option<Rate> {
        let t = text.trim();
        let body = t.strip_prefix('+').unwrap_or(t);
        if body.starts_with('-') {
            return None;
        }
        if let Some((n, d)) = body.split_once('/') {
            return Rate::new(digits(n)?, digits(d)?);
        }
        let (mantissa, exponent) = match body.split_once(['e', 'E']) {
            Some((m, e)) => (m, e.parse::<i32>().ok()?),
            None => (body, 0),
        };
        let (int_part, frac_part) = mantissa.split_once('.').unwrap_or((mantissa, ""));
        if int_part.is_empty() && frac_part.is_empty() {
            return None;
        }
        if !(int_part.is_empty() || int_part.bytes().all(|b| b.is_ascii_digit()))
            || !(frac_part.is_empty() || frac_part.bytes().all(|b| b.is_ascii_digit()))
            || int_part.len() + frac_part.len() > 30
            || exponent.abs() > 20
        {
            return None;
        }
        let mut num: u128 = format!("0{}{}", int_part, frac_part).parse().ok()?;
        let mut den: u128 = 10u128.checked_pow(frac_part.len() as u32)?;
        if exponent >= 0 {
            num = num.checked_mul(10u128.checked_pow(exponent as u32)?)?;
        } else {
            den = den.checked_mul(10u128.checked_pow((-exponent) as u32)?)?;
        }
        Rate::new(num, den)
    }

    /// The value as a float (one rounding, like Python's `float(Fraction)`).
    pub fn as_f64(&self) -> f64 {
        self.num as f64 / self.den as f64
    }

    /// The `N/D` text `vfx --fps`, `-framerate` and `fps=` take.
    pub fn text(&self) -> String {
        format!("{}/{}", self.num, self.den)
    }

    /// The pair for `neutral_ffmpeg`'s builders, which take `u32`s.
    pub fn as_u32_pair(&self) -> Option<(u32, u32)> {
        Some((u32::try_from(self.num).ok()?, u32::try_from(self.den).ok()?))
    }

    /// True when `self` and `base` differ by more than a thousandth of `base`
    /// (`abs(self - base) / base > Fraction(1, 1000)`), the variable frame rate test.
    pub fn differs_from(&self, base: &Rate) -> bool {
        let a = self.num as i128 * base.den as i128;
        let b = base.num as i128 * self.den as i128;
        let diff = (a - b).abs();
        // |an/ad - bn/bd| / (bn/bd) = |an*bd - bn*ad| / (ad*bn), compared with 1/1000.
        diff.saturating_mul(1000) > (self.den as i128).saturating_mul(base.num as i128)
    }
}

impl fmt::Display for Rate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.num, self.den)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_what_ffprobe_prints() {
        assert_eq!(Rate::parse("30000/1001").unwrap().text(), "30000/1001");
        assert_eq!(Rate::parse("24/1").unwrap().text(), "24/1");
        assert_eq!(Rate::parse("50/2").unwrap().text(), "25/1");
        assert_eq!(Rate::parse("7200/137").unwrap().text(), "7200/137");
        assert_eq!(Rate::parse("0/0"), None);
        assert_eq!(Rate::parse("0/1"), None);
        assert_eq!(Rate::parse("25/0"), None);
        assert_eq!(Rate::parse("N/A"), None);
        assert_eq!(Rate::parse(""), None);
        assert_eq!(Rate::parse("-25/1"), None);
    }

    #[test]
    fn parses_decimals_like_fraction() {
        assert_eq!(Rate::parse("29.97").unwrap().text(), "2997/100");
        assert_eq!(Rate::parse(" 24 ").unwrap().text(), "24/1");
        assert_eq!(Rate::parse("1e3").unwrap().text(), "1000/1");
        assert_eq!(Rate::parse("2.5E-1").unwrap().text(), "1/4");
        assert_eq!(Rate::parse(".5").unwrap().text(), "1/2");
        assert_eq!(Rate::parse("1."), Rate::new(1, 1));
        assert_eq!(Rate::parse("."), None);
        assert_eq!(Rate::parse("1/2/3"), None);
        assert_eq!(Rate::parse("a/b"), None);
    }

    #[test]
    fn float_conversion_is_a_single_division() {
        assert_eq!(Rate::new(24, 1).unwrap().as_f64(), 24.0);
        assert_eq!(Rate::new(30000, 1001).unwrap().as_f64(), 30000.0 / 1001.0);
    }

    #[test]
    fn variable_rate_threshold_is_a_thousandth_of_the_base() {
        let base = Rate::new(25, 1).unwrap();
        assert!(!Rate::new(25, 1).unwrap().differs_from(&base));
        assert!(!Rate::parse("24.98").unwrap().differs_from(&base), "0.08 percent is inside");
        assert!(Rate::parse("24.9").unwrap().differs_from(&base), "0.4 percent is outside");
        let exact_edge = Rate::new(24975, 1000).unwrap();
        assert!(!exact_edge.differs_from(&base), "exactly a thousandth is not more than a thousandth");
        assert!(Rate::new(7200, 137).unwrap().differs_from(&Rate::new(60, 1).unwrap()));
    }

    #[test]
    fn u32_pair_refuses_big_terms() {
        assert_eq!(Rate::new(30000, 1001).unwrap().as_u32_pair(), Some((30000, 1001)));
        assert_eq!(Rate::new(1 << 40, 1).unwrap().as_u32_pair(), None);
    }
}
