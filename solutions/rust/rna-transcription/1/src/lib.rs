#[derive(Debug, PartialEq, Eq)]
pub struct Dna(String);

#[derive(Debug, PartialEq, Eq)]
pub struct Rna(String);

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        for (index, symb) in dna.chars().enumerate() {
            if !['G', 'C', 'T', 'A'].contains(&symb) {
                return Err(index);
            }
        }

        Ok(Dna(dna.to_string()))
    }

    pub fn into_rna(self) -> Rna {
        let rna: String = self
            .0
            .chars()
            .map(|symb| match symb {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                'A' => 'U',
                _ => symb,
            })
            .collect();

        Rna(rna)
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        for (index, symb) in rna.chars().enumerate() {
            if !['C', 'G', 'A', 'U'].contains(&symb) {
                return Err(index);
            }
        }

        Ok(Rna(rna.to_string()))
    }
}
