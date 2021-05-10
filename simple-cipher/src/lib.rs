pub fn encode(key: &str, clear_text: &str) -> Option<String> {
    encode_decode(key, clear_text, key_char_to_encryption_key)
}

pub fn decode(key: &str, clear_text: &str) -> Option<String> {
    encode_decode(key, clear_text, key_char_to_decryption_key)
}

pub fn encode_random(clear_text: &str) -> (String, String) {
    let key = generate_random_key();
    (key.clone(), encode(key.as_str(), clear_text).unwrap())
}

pub fn encode_decode<F>(key: &str, clear_text: &str, key_mapper: F) -> Option<String>
where
    F: Fn(char) -> i8 + Clone,
{
    match key.is_empty()
        || key.contains(|c: char| !c.is_ascii_alphabetic() || c.is_ascii_uppercase())
    {
        true => None,
        false => Some(
            clear_text
                .chars()
                .filter(char::is_ascii_alphanumeric)
                .zip(
                    key.chars()
                        .flat_map(char::to_lowercase)
                        .map(key_mapper)
                        .cycle(),
                )
                .map(encrypt_char)
                .collect(),
        ),
    }
}

fn encrypt_char((byte_char_to_encrypt, key): (char, i8)) -> char {
    match byte_char_to_encrypt {
        'a'..='z' => {
            (((byte_char_to_encrypt as i8 - 'a' as i8 + key).rem_euclid(26)) as u8 + 'a' as u8)
                as char
        }
        'A'..='Z' => {
            (((byte_char_to_encrypt as i8 - 'A' as i8 + key).rem_euclid(26)) as u8 + 'A' as u8)
                as char
        }
        _ => byte_char_to_encrypt as char,
    }
}

fn key_char_to_encryption_key(key: char) -> i8 {
    (key as i8 - 'a' as i8) % 26
}

fn key_char_to_decryption_key(key: char) -> i8 {
    -(key as i8 - 'a' as i8) % 26
}

fn generate_random_key() -> String {
    (0..1024)
        .map(|_| (rand::random::<u8>() % 26 + 'a' as u8) as char)
        .collect()
}
