#![feature(destructuring_assignment)]

pub struct RailFence(usize);

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        Self(rails as usize)
    }

    pub fn encode(&self, plain_text: &str) -> String {
        let mut output = String::with_capacity(5 * plain_text.len() / 4);
        let rails = self.0;
        let text = plain_text.chars().collect::<Vec<char>>();
        for start in 0..rails {
            let skip1 = 2 * (rails - (start % (rails - 1) + 1));
            let skip2 = 2 * (rails - ((rails - start - 1) % (rails - 1) + 1));
            let mut offset = start;
            while offset < text.len() {
                output.push(text[offset]);
                offset += skip1;
                if offset < text.len() {
                    output.push(text[offset]);
                    offset += skip2;
                }
            }
        }
        output
    }

    pub fn decode(&self, cipher_text: &str) -> String {
        let (rails, cipher_chars, num_chars_on_rail) =
            self.compute_decoding_preliminaries(cipher_text);
        Self::decode_cipher_chars(rails, cipher_chars, num_chars_on_rail)
    }

    fn decode_cipher_chars(
        rails: usize,
        cipher_chars: Vec<char>,
        num_chars_on_rail: Vec<(usize, usize)>,
    ) -> String {
        let mut output = String::with_capacity(5 * cipher_chars.len() / 4);
        let mut cycle_count = -1_isize;
        let mut cycle_is_downward = true;
        for (rail, _) in (0..rails)
            .chain((1..(rails - 1)).rev())
            .cycle()
            .zip(0..cipher_chars.len())
        {
            (cycle_count, cycle_is_downward) =
                Self::compute_decoding_cycle_params_at_cycle_beginning(
                    rail,
                    cycle_count,
                    cycle_is_downward,
                );
            output.push(
                cipher_chars[Self::compute_decoding_char_to_use(
                    rail,
                    &num_chars_on_rail,
                    cycle_is_downward,
                    cycle_count,
                )],
            );
            (cycle_count, cycle_is_downward) = Self::compute_decoding_cycle_params_at_cycle_ending(
                rail,
                rails,
                cycle_count,
                cycle_is_downward,
            );
        }
        output
    }

    fn compute_decoding_char_to_use(
        rail: usize,
        num_chars_on_rail: &Vec<(usize, usize)>,
        cycle_is_downward: bool,
        cycle_count: isize,
    ) -> usize {
        let skip_prior_rails_letters_count =
            (0..rail).map(|r| num_chars_on_rail[r].1).sum::<usize>();
        let chars_this_rail_to_skip_count = match cycle_is_downward {
            true => cycle_count as usize * num_chars_on_rail[rail].0,
            false => cycle_count as usize * num_chars_on_rail[rail].0 + 1,
        };
        skip_prior_rails_letters_count + chars_this_rail_to_skip_count
    }

    fn compute_decoding_preliminaries(
        &self,
        cipher_text: &str,
    ) -> (usize, Vec<char>, Vec<(usize, usize)>) {
        let rails = self.0;
        let cipher_chars = cipher_text.chars().collect::<Vec<char>>();
        let chars_per_cycle = rails * 2 - 2;
        let number_of_cycles = cipher_chars.len() as f32 / chars_per_cycle as f32;
        let num_chars_on_rail =
            Self::compute_number_of_characters_for_each_rail(rails, number_of_cycles);
        (rails, cipher_chars, num_chars_on_rail)
    }

    fn compute_number_of_characters_for_each_rail(
        rails: usize,
        number_of_cycles: f32,
    ) -> Vec<(usize, usize)> {
        let mut num_chars_on_rail = Vec::<(usize, usize)>::with_capacity(rails);
        for rail in 0..rails {
            num_chars_on_rail.push(match rail {
                0 => (1, number_of_cycles.ceil() as usize),
                rail if rail < rails - 1 => (2, (number_of_cycles * 2_f32).round() as usize),
                _ => (1, number_of_cycles.floor() as usize),
            });
        }
        num_chars_on_rail
    }

    fn compute_decoding_cycle_params_at_cycle_beginning(
        rail: usize,
        cycle_count: isize,
        cycle_is_downward: bool,
    ) -> (isize, bool) {
        match rail {
            0 => (cycle_count + 1, true),
            _ => (cycle_count, cycle_is_downward),
        }
    }

    fn compute_decoding_cycle_params_at_cycle_ending(
        rail: usize,
        rails: usize,
        cycle_count: isize,
        cycle_is_downward: bool,
    ) -> (isize, bool) {
        match rail {
            rail if rail == rails - 1 => (cycle_count, false),
            _ => (cycle_count, cycle_is_downward),
        }
    }
}
