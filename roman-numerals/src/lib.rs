use std::fmt::{Display, Formatter, Result};

pub struct Roman {
    thousands: u32,
    hundreds: u32,
    tens: u32,
    ones: u32,
}

impl Roman {
    fn format_thousands(&self) -> &str {
        match self.thousands {
            0 => "",
            n @ 1..=3 => &"MMM"[0..n as usize],
            _ => "?",
        }
    }
    fn format_hundreds(&self) -> &str {
        match self.hundreds {
            0 => "",
            n @ 1..=3 => &"CCC"[0..n as usize],
            4 => "CD",
            n @ 5..=8 => &"DCCC"[0..(n as usize - 4)],
            9 => "CM",
            _ => "?",
        }
    }
    fn format_tens(&self) -> &str {
        match self.tens {
            0 => "",
            n @ 1..=3 => &"XXX"[0..n as usize],
            4 => "XL",
            n @ 5..=8 => &"LXXX"[0..(n as usize - 4)],
            9 => "XC",
            _ => "?",
        }
    }
    fn format_ones(&self) -> &str {
        match self.ones {
            0 => "",
            n @ 1..=3 => &"III"[0..n as usize],
            4 => "IV",
            n @ 5..=8 => &"VIII"[0..(n as usize - 4)],
            9 => "IX",
            _ => "?",
        }
    }
}

impl Display for Roman {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        let formatted = String::with_capacity(20)
            + self.format_thousands()
            + self.format_hundreds()
            + self.format_tens()
            + self.format_ones();
        formatter.write_str(formatted.as_str())
    }
}

impl From<u32> for Roman {
    fn from(mut num: u32) -> Self {
        let thousands = num / 1000;
        num -= thousands * 1000;
        let hundreds = num / 100;
        num -= hundreds * 100;
        let tens = num / 10;
        num -= tens * 10;
        let ones = num;
        Self {
            thousands,
            hundreds,
            tens,
            ones,
        }
    }
}
