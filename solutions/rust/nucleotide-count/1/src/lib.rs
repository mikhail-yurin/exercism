use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let mut count: usize = 0;

    for upper_nucleotide in nucleotide.to_uppercase() {
        if !['A', 'C', 'G', 'T'].contains(&upper_nucleotide) {
            return Err(upper_nucleotide);
        }

        for upper_dna_part in dna.to_uppercase().chars() {
            if !['A', 'C', 'G', 'T'].contains(&upper_dna_part) {
                return Err(upper_dna_part);
            }

            if upper_dna_part == upper_nucleotide {
                count += 1;
            }
        }
    }

    Ok(count)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut map: HashMap<char, usize> = HashMap::from([('A', 0), ('C', 0), ('G', 0), ('T', 0)]);

    for symb in dna.chars() {
        for upper_nucleotide in symb.to_uppercase() {
            if !['A', 'C', 'G', 'T'].contains(&upper_nucleotide) {
                return Err(upper_nucleotide);
            }

            if let Some(count) = map.get(&upper_nucleotide) {
                map.insert(upper_nucleotide, count + 1);
            }
        }
    }

    Ok(map)
}
