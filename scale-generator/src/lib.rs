#![feature(iter_map_while)]

use std::iter;

#[derive(Debug)]
pub enum Error {
    InvalidTonic,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Scale<'a> {
    notes: Vec<&'a str>,
}

static SHARP_SCALE: &[&str] = &[
    "A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#",
];
static SHARP_MINOR_SCALE: &[&str] = &[
    "a", "a#", "b", "c", "c#", "d", "d#", "e", "f", "f#", "g", "g#",
];
static FLAT_SCALE: &[&str] = &[
    "A", "Bb", "B", "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab",
];
static FLAT_MINOR_SCALE: &[&str] = &[
    "a", "bb", "b", "c", "db", "d", "eb", "e", "f", "gb", "g", "ab",
];

impl<'a> Scale<'a> {
    pub fn new<'b>(tonic: &'b str, intervals: &'b str) -> Result<Scale<'a>, Error>
    where
        'b: 'a,
    {
        let mut ordered_chromatic_scale = Self::scale_slices(tonic)?;
        match intervals {
            "m" => Ok(Self {
                notes: ordered_chromatic_scale.collect(),
            }),
            intervals => Ok(Self {
                notes: iter::once('m')
                    .chain(intervals.chars())
                    .cycle()
                    .map_while(|interval| match interval {
                        'm' => ordered_chromatic_scale.next(),
                        'M' => ordered_chromatic_scale.nth(1),
                        'A' => ordered_chromatic_scale.nth(2),
                        _ => None,
                    })
                    .collect(),
            }),
        }
    }

    pub fn chromatic<'b>(tonic: &'b str) -> Result<Scale<'a>, Error>
    where
        'b: 'a,
    {
        Self::new(tonic, "m")
    }

    pub fn enumerate(&self) -> Vec<String> {
        self.notes.iter().map(|s| s.to_string()).collect()
    }

    fn scale_slices<'b>(tonic: &'b str) -> Result<impl Iterator<Item = &str> + 'b, Error>
    where
        'a: 'b,
    {
        if let Some((rest_slice, first_slice)) = match tonic {
            "C" | "G" | "D" | "A" | "E" | "B" | "F#" => SHARP_SCALE
                .iter()
                .position(|note| *note == tonic)
                .map(|note_position| SHARP_SCALE.split_at(note_position)),
            "a" | "e" | "b" | "f#" | "c#" | "g#" | "d#" => SHARP_MINOR_SCALE
                .iter()
                .position(|note| *note == tonic)
                .map(|note_position| SHARP_SCALE.split_at(note_position)),
            "F" | "Bb" | "Eb" | "Ab" | "Db" | "Gb" => FLAT_SCALE
                .iter()
                .position(|note| *note == tonic)
                .map(|note_position| FLAT_SCALE.split_at(note_position)),
            "d" | "g" | "c" | "f" | "bb" | "eb" => FLAT_MINOR_SCALE
                .iter()
                .position(|note| *note == tonic)
                .map(|note_position| FLAT_SCALE.split_at(note_position)),
            _ => None,
        } {
            Ok(first_slice.iter().chain(rest_slice.iter()).copied())
        } else {
            Err(Error::InvalidTonic)
        }
    }
}
