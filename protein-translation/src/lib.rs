use std::collections::HashMap;

pub struct CodonsInfo<'a>(HashMap<&'a str, &'a str>);

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        self.0.get(codon).map(|protein| *protein)
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        let mut proteins = Vec::new();
        'PROCESSING_CODONS: for idx in (0..rna.len()).step_by(3) {
            if rna.len() < idx + 3 {
                return None;
            }
            let codon = &rna[idx..(idx + 3)];
            let protein = self.name_for(codon);
            match protein {
                Some("stop codon") => break 'PROCESSING_CODONS,
                Some(protein) => proteins.push(protein),
                None => return None,
            }
        }
        Some(proteins)
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    CodonsInfo(pairs.into_iter().collect())
}
