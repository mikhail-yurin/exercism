use rayon::prelude::*;
use std::char;
use std::collections::HashMap;

type CharMap = HashMap<char, u8>;

struct Permutations {
    unique_chars: Vec<char>,
    non_zeros: Vec<char>,
    operations: i64,
    skipped: i64,
}

impl Permutations {
    fn generate_permutations(&mut self, digits: Vec<u8>, n: usize) -> Vec<Vec<u8>> {
        self.operations += 1;

        if n == 1 {
            return digits.iter().map(|el| vec![*el]).collect();
        }

        let mut permutations: Vec<Vec<u8>> = Vec::new();

        'generate_with_digit: for (index, digit) in digits.iter().enumerate() {
            // let remaining = [&digits[0..index], &digits[index + 1..]].concat();
            let remaining: Vec<u8> = digits
                .iter()
                .enumerate()
                .filter(|&(i, _)| i != index)
                .map(|(_, &d)| d)
                .collect();

            if self
                .non_zeros
                .contains(&self.unique_chars[self.unique_chars.len() - n])
                && digit == &0
            {
                self.skipped += 1;
                continue 'generate_with_digit;
            }

            let sub_permutations = self.generate_permutations(remaining, n - 1);

            for mut sub_permutation in sub_permutations {
                sub_permutation.insert(0, *digit);

                permutations.push(sub_permutation);
            }
        }

        permutations
    }
}

fn check_equation(equasion: &str, permutation: &[u8], unique_chars: &[char]) -> bool {
    // Generate ASCII char map
    let mut char_map = [0u8; 26];
    for (index, &char) in unique_chars.iter().enumerate() {
        char_map[char as usize - 65] = permutation[index];
    }

    // Split equation
    let equation_parts: Vec<&str> = equasion.split("==").collect();

    // Result
    let mut result = 0u64;
    for char in equation_parts[1].chars() {
        result = result * 10 + char_map[char as usize - 65] as u64;
    }

    // Sum
    let mut sum = 0u64;
    let mut current_number = 0u64;
    for char in equation_parts[0].chars() {
        match char {
            c if c.is_alphabetic() => {
                current_number = current_number * 10 + char_map[char as usize - 65] as u64;
            }
            '+' => {
                sum += current_number;
                current_number = 0;
            }
            _ => {}
        }
    }
    sum += current_number; // add the last number to sum

    sum == result
}

pub fn solve(input: &str) -> Option<CharMap> {
    let no_spaces = input.replace(" ", "");

    let mut unique_chars: Vec<char> = Vec::new();
    let mut non_zeros: Vec<char> = Vec::new();

    let mut prev_char: char = ' ';

    for char in no_spaces.chars() {
        if char.is_alphabetic() {
            if !unique_chars.contains(&char) {
                unique_chars.push(char);
            }

            if !non_zeros.contains(&char) && !prev_char.is_alphabetic() {
                non_zeros.push(char);
            }
        }

        prev_char = char;
    }

    let unique_chars_number: usize = unique_chars.len();
    let digits = Vec::from([9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);

    let mut permutations_struct = Permutations {
        unique_chars,
        non_zeros,
        operations: 0,
        skipped: 0,
    };

    let permutations = permutations_struct.generate_permutations(digits, unique_chars_number);

    let result = permutations.into_par_iter().find_any(|permutation| {
        check_equation(&no_spaces, permutation, &permutations_struct.unique_chars)
    });

    if let Some(permutation) = result {
        let mut char_map: CharMap = HashMap::new();
        for (index, char) in permutations_struct.unique_chars.iter().enumerate() {
            let value = *permutation.get(index).unwrap();
            char_map.insert(*char, value);
        }
        return Some(char_map);
    }

    None
}
