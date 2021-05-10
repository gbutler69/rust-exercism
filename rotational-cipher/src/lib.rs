pub fn rotate(input: &str, key: i8) -> String {
    let key = adjust_key_to_plus_or_minus_13(key);
    let encode = |b| encode(b, key);
    input.chars().map(encode).collect()
}

fn encode(byte_char_to_encrypt: char, key: i8) -> char {
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

fn adjust_key_to_plus_or_minus_13(key: i8) -> i8 {
    key % 26
}
