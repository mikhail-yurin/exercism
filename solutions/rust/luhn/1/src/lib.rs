use regex::Regex;

/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let re = Regex::new(r"^[\d\s]+$").unwrap();

    if !re.is_match(code) {
        return false;
    }

    let trimmed = code.replace(" ", "");

    if trimmed.len() < 2 {
        return false;
    }

    let use_even = trimmed.len() % 2 == 0;

    let numbers: Vec<u32> = trimmed
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
