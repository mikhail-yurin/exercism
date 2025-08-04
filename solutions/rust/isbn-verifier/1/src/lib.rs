/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let replaced = if isbn.contains("-") {
        isbn.replace("-", "")
    } else {
        isbn.to_string()
    };

    if replaced.len() != 10 {
        return false;
    }

    let mut sum: u32 = 0;

    for (index, char) in replaced.chars().rev().enumerate() {
        if let Some(digit) = char.to_digit(10) {
            sum += digit * (index as u32 + 1);
        } else if char == 'X' && index == 0 {
            sum += 10;
        } else {
            return false;
        }
    }

    sum % 11 == 0
}
