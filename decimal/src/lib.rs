use std::{
    ops::{Add, Mul, Sub},
    str::FromStr,
};

use num_bigint::BigInt;

/// Type implementing arbitrary-precision decimal arithmetic
#[derive(Clone, Debug)]
pub struct Decimal {
    digits: BigInt,
    scale: u32,
}

impl Decimal {
    pub fn try_from(input: &str) -> Option<Self> {
        let (whole_digits, fractional_digits) =
            if let Some((whole_digits, fractional_digits)) = input.split_once('.') {
                (whole_digits, fractional_digits)
            } else {
                (input, "")
            };
        let fractional_digits =
            fractional_digits.trim_end_matches(|c: char| c == '0' || c.is_whitespace());
        match (
            BigInt::from_str(whole_digits),
            BigInt::from_str(fractional_digits),
        ) {
            (Ok(whole), Ok(fractional)) => {
                let scale = fractional_digits.len() as u32;
                let sign = BigInt::from(if whole_digits.starts_with('-') { -1 } else { 1 });
                let mut digits = whole * BigInt::from(10).pow(scale);
                digits += sign * fractional;
                Some(Self { digits, scale })
            }
            (Ok(whole), _) if fractional_digits.is_empty() => Some(Self {
                digits: whole,
                scale: 0,
            }),
            _ => None,
        }
    }

    fn scale_same(self, rhs: Decimal) -> (BigInt, BigInt, u32) {
        let (lhs, rhs, scale) = match (self.scale, rhs.scale) {
            (lscale, rscale) if lscale > rscale => (
                self.digits,
                rhs.digits * BigInt::from(10).pow(lscale - rscale),
                lscale,
            ),
            (lscale, rscale) if lscale < rscale => (
                self.digits * BigInt::from(10).pow(rscale - lscale),
                rhs.digits,
                rscale,
            ),
            (lscale, _) => (self.digits, rhs.digits, lscale),
        };
        (lhs, rhs, scale)
    }
}

impl Add<Decimal> for Decimal {
    type Output = Decimal;

    fn add(self, rhs: Decimal) -> Self::Output {
        let (lhs, rhs, scale) = self.scale_same(rhs);
        Self {
            digits: lhs + rhs,
            scale,
        }
    }
}

impl Sub<Decimal> for Decimal {
    type Output = Decimal;

    fn sub(self, rhs: Decimal) -> Self::Output {
        let (lhs, rhs, scale) = self.scale_same(rhs);
        Self {
            digits: lhs - rhs,
            scale,
        }
    }
}

impl Mul<Decimal> for Decimal {
    type Output = Decimal;

    fn mul(self, rhs: Decimal) -> Self::Output {
        Self {
            digits: self.digits * rhs.digits,
            scale: self.scale + rhs.scale,
        }
    }
}

impl PartialEq<Decimal> for Decimal {
    fn eq(&self, other: &Decimal) -> bool {
        let (lhs, rhs, _) = self.clone().scale_same(other.clone());
        lhs == rhs
    }
}

impl Eq for Decimal {}

impl PartialOrd<Decimal> for Decimal {
    fn partial_cmp(&self, other: &Decimal) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Decimal {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let (lhs, rhs, _) = self.clone().scale_same(other.clone());
        lhs.cmp(&rhs)
    }
}
