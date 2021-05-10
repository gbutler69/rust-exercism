use std::time::{SystemTime, UNIX_EPOCH};

// This will use a pseudo-random, hand-written function rather than something properly
// cryptographically secure like that provided by the rand crate (for fun)
// DO NOT DO THIS IN A REAL IMPLEMENTATION!!!!!
pub fn private_key(p: u64) -> u64 {
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let rand = seed as u128 ^ (seed as u128) << 11;
    let rand = ((rand ^ (rand >> 15)) as u64 as u128).pow(2) as u64;
    let rand = (rand as f64) / u64::MAX as f64;
    (((p as f64) * rand).round() as u64).clamp(2, p - 1)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    modular_exponentiate(g, a, p)
}

#[cfg(feature = "slow-modular-exponentiate")]
/// This is a slow version of modular exponentiation (use the feature flag to toggle between fast and slow to compare)
fn modular_exponentiate(base: u64, exponent: u64, modulus: u64) -> u64 {
    let (base, modulus) = (base as u128, modulus as u128);
    if modulus == 1 {
        return 0;
    }
    let mut c = 1_u128;
    for _ in 0..exponent {
        c = (c * base) % modulus
    }
    c as u64
}

/// This is the fast version
#[cfg(not(feature = "slow-modular-exponentiate"))]
fn modular_exponentiate(base: u64, exponent: u64, modulus: u64) -> u64 {
    let (mut base, mut exponent, modulus) = (base as u128, exponent as u128, modulus as u128);
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    base %= modulus;
    while exponent > 0 {
        if exponent % 2 == 1 {
            result = ((result % modulus) * (base % modulus)) % modulus;
        }
        exponent >>= 1;
        base = (base % modulus).pow(2) % modulus;
    }
    result as u64
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    modular_exponentiate(b_pub, a, p)
}
