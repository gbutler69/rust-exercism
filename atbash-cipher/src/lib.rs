use itertools::Itertools;

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    plain
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .flat_map(char::to_lowercase)
        .map(atbash_cipher)
        .chunks(5)
        .into_iter()
        .map(|group| group.collect::<String>())
        .join(" ")
}

fn atbash_cipher(char_to_encrypt: char) -> char {
    if char_to_encrypt.is_numeric() {
        return char_to_encrypt;
    }
    ('z' as u8 - char_to_encrypt as u8 + 'a' as u8) as char
}
/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .flat_map(char::to_lowercase)
        .map(atbash_cipher)
        .collect()
}
