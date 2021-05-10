#![feature(destructuring_assignment)]

use itertools::Itertools;

const ALPHABET_LENGTH: i32 = 26;

/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, key_a: i32, key_b: i32) -> Result<String, AffineCipherError> {
    match find_gcd_and_bezout_coefficients(key_a, ALPHABET_LENGTH) {
        GCDAndBezoutCoefficients { gcd: 1, .. } => Ok(encrypt(plaintext, key_a, key_b)),
        _ => Err(AffineCipherError::NotCoprime(key_a)),
    }
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, key_a: i32, key_b: i32) -> Result<String, AffineCipherError> {
    match find_gcd_and_bezout_coefficients(key_a, ALPHABET_LENGTH) {
        GCDAndBezoutCoefficients {
            gcd: 1,
            bezout_coeff_x: key_a_mmi,
            ..
        } => Ok(decrypt(ciphertext, key_a_mmi, key_b)),
        _ => Err(AffineCipherError::NotCoprime(key_a)),
    }
}

fn encrypt(plaintext: &str, key_a: i32, key_b: i32) -> String {
    plaintext
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .flat_map(char::to_lowercase)
        .map(|char_to_encrypt| encrypt_char(char_to_encrypt, key_a, key_b))
        .chunks(5)
        .into_iter()
        .map(|iter_5_chars_at_a_time| iter_5_chars_at_a_time.chain(std::iter::once(' ')))
        .flatten()
        .collect::<String>()
        .trim()
        .into()
}

fn encrypt_char(char_to_encrypt: char, key_a: i32, key_b: i32) -> char {
    if char_to_encrypt.is_numeric() {
        return char_to_encrypt;
    }
    let x = char_to_letter_num_of_alphabet(char_to_encrypt);
    let encrypted_x = (key_a * x + key_b).rem_euclid(ALPHABET_LENGTH) as u8;
    letter_num_of_alphabet_to_char(encrypted_x)
}

fn decrypt(ciphertext: &str, key_a_mmi: i32, key_b: i32) -> String {
    ciphertext
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .flat_map(char::to_lowercase)
        .map(|char_to_decrypt| decrypt_char(char_to_decrypt, key_a_mmi, key_b))
        .collect::<String>()
}

fn decrypt_char(char_to_decrypt: char, key_a_mmi: i32, key_b: i32) -> char {
    if char_to_decrypt.is_numeric() {
        return char_to_decrypt;
    }
    let y = char_to_letter_num_of_alphabet(char_to_decrypt);
    let decrypted_y = (key_a_mmi * (y - key_b)).rem_euclid(ALPHABET_LENGTH) as u8;
    letter_num_of_alphabet_to_char(decrypted_y)
}

struct GCDAndBezoutCoefficients {
    gcd: i32,
    bezout_coeff_x: i32,
}

fn find_gcd_and_bezout_coefficients(a: i32, b: i32) -> GCDAndBezoutCoefficients {
    let (mut old_r, mut r) = (a, b);
    let (mut old_s, mut s) = (1, 0);
    while r != 0 {
        let q = old_r / r;
        (old_r, r) = (r, old_r - q * r);
        (old_s, s) = (s, old_s - q * s);
    }
    GCDAndBezoutCoefficients {
        gcd: old_r,
        bezout_coeff_x: old_s,
    }
}

fn char_to_letter_num_of_alphabet(c: char) -> i32 {
    (c as u8 - 'a' as u8) as i32
}

fn letter_num_of_alphabet_to_char(letter_number: u8) -> char {
    (letter_number + 'a' as u8) as char
}

// TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS
// TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS
// TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS
// TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS
// TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS

#[test]
fn test_find_gcd_and_bezout_coefficients_mmi_for_9_mod_26() {
    assert_eq!(find_gcd_and_bezout_coefficients(9, 26).bezout_coeff_x, 3);
}

#[test]
fn test_find_gcd_and_bezout_coefficients_mmi_for_15_mod_26() {
    assert_eq!(find_gcd_and_bezout_coefficients(15, 26).bezout_coeff_x, 7);
}

#[test]
fn test_find_gcd_and_bezout_coefficients_mmi_for_3_mod_7() {
    assert_eq!(find_gcd_and_bezout_coefficients(3, 26).bezout_coeff_x, 9);
}
