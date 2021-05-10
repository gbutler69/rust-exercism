/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    sentence
        .chars()
        .filter(char::is_ascii_alphabetic)
        .flat_map(char::to_uppercase)
        .fold(SimpleBitVec::<4>::new(), |mut bitvec, letter| {
            bitvec.set(letter as usize - 'A' as usize);
            bitvec
        })
        .get_first_unset()
        .map_or(false, |first_unset| first_unset == 26)
}

struct SimpleBitVec<const SIZE_DIV_8: usize> {
    bits: [u8; SIZE_DIV_8],
}

impl<const SIZE_DIV_8: usize> SimpleBitVec<SIZE_DIV_8> {
    const SIZE: usize = SIZE_DIV_8 * 8;
    pub fn new() -> Self {
        SimpleBitVec {
            bits: [0; SIZE_DIV_8],
        }
    }
    pub fn set(&mut self, bit: usize) {
        let (byte_addr, bit_addr) = self.compute_address_components(bit);
        self.bits[byte_addr] |= (1 << bit_addr) as u8
    }
    pub fn get(&self, bit: usize) -> bool {
        let (byte_addr, bit_addr) = self.compute_address_components(bit);
        self.bits[byte_addr] & (1 << bit_addr) as u8 != 0
    }
    pub fn get_first_unset(&self) -> Option<usize> {
        for i in 0..Self::SIZE {
            if let false = self.get(i) {
                return Some(i);
            }
        }
        None
    }
    fn compute_address_components(&self, bit: usize) -> (usize, u8) {
        if bit >= Self::SIZE {
            panic!("Invalid Address");
        }
        (bit / 8, (bit % 8) as u8)
    }
}

// TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS
// TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS
// TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS
// TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS
// TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS

#[test]
fn test_simple_bitvec_1() {
    let mut bitvec = SimpleBitVec::<1>::new();
    bitvec.set(0);
    bitvec.set(2);
    bitvec.set(3);
    bitvec.set(7);
    assert_eq!(true, bitvec.get(0));
    assert_eq!(false, bitvec.get(1));
    assert_eq!(true, bitvec.get(2));
    assert_eq!(true, bitvec.get(3));
    assert_eq!(false, bitvec.get(4));
    assert_eq!(false, bitvec.get(5));
    assert_eq!(false, bitvec.get(6));
    assert_eq!(true, bitvec.get(7));
}

#[test]
#[should_panic(expected = "Invalid Address")]
fn test_simple_bitvec_1_out_of_range_on_set() {
    let mut bitvec = SimpleBitVec::<1>::new();
    bitvec.set(8);
}

#[test]
#[should_panic(expected = "Invalid Address")]
fn test_simple_bitvec_1_out_of_range_on_get() {
    let bitvec = SimpleBitVec::<1>::new();
    bitvec.get(8);
}

#[test]
fn test_simple_bitvec_12() {
    let mut bitvec = SimpleBitVec::<12>::new();
    for bit in 0..(12 * 8) {
        if bit % 3 == 0 {
            bitvec.set(bit);
        }
    }
    for bit in 0..(12 * 8) {
        if bit % 3 == 0 {
            assert_eq!(true, bitvec.get(bit));
        } else {
            assert_eq!(false, bitvec.get(bit));
        }
    }
    assert_eq!(true, bitvec.get(0));
    assert_eq!(true, bitvec.get(3));
    assert_eq!(true, bitvec.get(36));
    assert_eq!(true, bitvec.get(60));
    assert_eq!(true, bitvec.get(90));
}
