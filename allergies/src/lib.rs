#![feature(custom_inner_attributes)]

pub struct Allergies(u32);

#[derive(Debug, PartialEq)]
#[repr(u32)]
#[non_exhaustive]
pub enum Allergen {
    Eggs         = 0x01,
    Peanuts      = 0x02,
    Shellfish    = 0x04,
    Strawberries = 0x08,
    Tomatoes     = 0x10,
    Chocolate    = 0x20,
    Pollen       = 0x40,
    Cats         = 0x80,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self(score)
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        #![rustfmt::skip]
        #[allow(unreachable_patterns)]
        match allergen {
            Allergen::Eggs         => self.0 & Allergen::Eggs         as u32 != 0,
            Allergen::Peanuts      => self.0 & Allergen::Peanuts      as u32 != 0,
            Allergen::Shellfish    => self.0 & Allergen::Shellfish    as u32 != 0,
            Allergen::Strawberries => self.0 & Allergen::Strawberries as u32 != 0,
            Allergen::Tomatoes     => self.0 & Allergen::Tomatoes     as u32 != 0,
            Allergen::Chocolate    => self.0 & Allergen::Chocolate    as u32 != 0,
            Allergen::Pollen       => self.0 & Allergen::Pollen       as u32 != 0,
            Allergen::Cats         => self.0 & Allergen::Cats         as u32 != 0,
            _                      => false
        }
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        (0..=7)
            .map(|i| 2_u32.pow(i))
            .map(Self::num_to_allergen_or_none_if_invalid)
            .flatten()
            .map(|allergen| self.to_allergy_if_allergic_or_none_if_not(allergen))
            .flatten()
            .collect()
    }

    fn to_allergy_if_allergic_or_none_if_not(&self, allergen: Allergen) -> Option<Allergen> {
        match self.is_allergic_to(&allergen) {
            true => Some(allergen),
            false => None,
        }
    }

    fn num_to_allergen_or_none_if_invalid(allergen_number: u32) -> Option<Allergen> {
        match allergen_number {
            0x01 => Some(Allergen::Eggs),
            0x02 => Some(Allergen::Peanuts),
            0x04 => Some(Allergen::Shellfish),
            0x08 => Some(Allergen::Strawberries),
            0x10 => Some(Allergen::Tomatoes),
            0x20 => Some(Allergen::Chocolate),
            0x40 => Some(Allergen::Pollen),
            0x80 => Some(Allergen::Cats),
            _ => None,
        }
    }
}
