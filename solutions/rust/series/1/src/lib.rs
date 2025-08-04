pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut numbers: Vec<String> = Vec::new();

    let dlen = digits.len();
    if dlen < len {
        return numbers;
    }

    for i in 0..=dlen - len {
        let mut number: String = String::new();

        for j in i..i + len {
            number.push(digits.chars().nth(j).unwrap());
        }

        numbers.push(number);
    }

    numbers
}
