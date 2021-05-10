use std::{collections::HashSet, sync::RwLock};

use lazy_static::lazy_static;

pub struct Robot(String);

lazy_static! {
    static ref USED_ROBOT_NAMES: RwLock<HashSet<String>> = RwLock::new(HashSet::new());
}

impl Robot {
    pub fn new() -> Self {
        Self(Self::random_name())
    }

    pub fn name(&self) -> &str {
        &self.0
    }

    pub fn reset_name(&mut self) {
        self.0 = Self::random_name();
    }

    fn random_name() -> String {
        'TRY_FIND_UNUSED_NAME_AND_RECORD_AS_USED: loop {
            let mut name;
            'FIND_UNUSED_NAME: loop {
                let letter1 = Self::random_letter_A_to_Z();
                let letter2 = Self::random_letter_A_to_Z();
                let digits = Self::random_3_digit_number();
                name = format!("{}{}{:03}", letter1, letter2, digits);
                match USED_ROBOT_NAMES.read() {
                    Ok(read_locked_used_names_map) => {
                        match read_locked_used_names_map.contains(&name) {
                            true => continue 'FIND_UNUSED_NAME,
                            false => break,
                        }
                    }
                    Err(_) => {
                        panic!(
                            "RwLock for already used robot names poisoned - ABORTING APPLICATION"
                        )
                    }
                }
            }
            match USED_ROBOT_NAMES.write() {
                Ok(mut write_locked_used_names_map) => {
                    match write_locked_used_names_map.insert(name.clone()) {
                        true => return name,
                        false => continue 'TRY_FIND_UNUSED_NAME_AND_RECORD_AS_USED,
                    }
                }
                Err(_) => {
                    panic!("RwLock for already used robot names poisoned - ABORTING APPLICATION")
                }
            }
        }
    }

    #[allow(non_snake_case)]
    fn random_letter_A_to_Z() -> char {
        (rand::random::<u8>() % 26 + 'A' as u8) as char
    }

    fn random_3_digit_number() -> u16 {
        rand::random::<u16>() % 1000
    }
}
