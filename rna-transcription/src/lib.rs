use std::hint::unreachable_unchecked;

#[derive(Debug, PartialEq)]
pub struct Dna {
    nucleotides: String,
}

#[derive(Debug, PartialEq)]
pub struct Rna {
    nucleotides: String,
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        match dna
            .chars()
            .enumerate()
            .map(Self::invalid_nucleotide_to_error)
            .collect()
        {
            Ok(s) => Ok(Self { nucleotides: s }),
            Err(idx) => Err(idx),
        }
    }

    fn invalid_nucleotide_to_error((idx, c): (usize, char)) -> Result<char, usize> {
        match c {
            'A' | 'C' | 'G' | 'T' => Ok(c),
            _ => Err(idx),
        }
    }

    pub fn into_rna(self) -> Rna {
        Rna {
            nucleotides: self
                .nucleotides
                .chars()
                .into_iter()
                .map(Self::dna_nucleotide_to_rna_complement)
                .collect(),
        }
    }

    fn dna_nucleotide_to_rna_complement(dna_nucleotide: char) -> char {
        match dna_nucleotide {
            'G' => 'C',
            'C' => 'G',
            'T' => 'A',
            'A' => 'U',
            _ => unsafe { unreachable_unchecked() },
        }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        match rna
            .chars()
            .enumerate()
            .map(Self::invalid_nucleotide_to_error)
            .collect()
        {
            Ok(s) => Ok(Self { nucleotides: s }),
            Err(idx) => Err(idx),
        }
    }

    fn invalid_nucleotide_to_error((idx, c): (usize, char)) -> Result<char, usize> {
        match c {
            'C' | 'G' | 'A' | 'U' => Ok(c),
            _ => Err(idx),
        }
    }
}
