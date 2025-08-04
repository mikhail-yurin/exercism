use regex::Regex;
use std::convert::From;

pub struct Luhn {
    data: String,
}

impl<T: ToString> From<T> for Luhn {
    fn from(input: T) -> Self {
        let data = input.to_string().replace(" ", "");
        Luhn { data }
    }
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        let re = Regex::new(r"^[\d\s]+$").unwrap();
        if self.data.len() < 2 || !re.is_match(&self.data) {
            return false;
        }

        let use_even = self.data.len() % 2 == 0;
        let numbers: Vec<u32> = self
            .data
            .chars()
            .enumerate()
            .map(|(i, char)| {
                let digit = char.to_digit(10).unwrap();
                if (use_even && i % 2 == 0) || (!use_even && i % 2 != 0) {
                    let mut calculated = 2 * digit;
                    if calculated > 9 {
                        calculated -= 9;
                    }
                    return calculated;
                }
                digit
            })
            .collect();

        let sum: u32 = numbers.iter().sum();
        sum % 10 == 0
    }
}
