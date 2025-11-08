fn get_counter_string(counter: usize) -> String {
    if counter > 1 {
        counter.to_string()
    } else {
        String::new()
    }
}

pub fn encode(source: &str) -> String {
    let mut result = String::new();
    if source.is_empty() {
        return result;
    }

    let mut chars = source.chars();
    let mut counter: usize = 0;
    let mut prev_symbol: Option<char> = None;

    loop {
        let current = chars.next();

        match current {
            Some(current_symbol) => {
                if prev_symbol.is_none() || current_symbol == prev_symbol.unwrap() {
                    counter += 1;
                } else {
                    result = format!(
                        "{result}{}{}",
                        get_counter_string(counter),
                        prev_symbol.unwrap()
                    );
                    counter = 1;
                }

                prev_symbol = Some(current_symbol);
            }
            None => {
                result = format!(
                    "{result}{}{}",
                    get_counter_string(counter),
                    prev_symbol.unwrap()
                );
                break;
            }
        }
    }

    result
}

pub fn decode(source: &str) -> String {
    let mut chars = source.chars();
    let mut result = String::new();
    let mut number: u32 = 0;

    loop {
        let current = chars.next();

        match current {
            Some(current_symbol) => {
                let parsed = current_symbol.to_digit(10);

                match parsed {
                    Some(digit) => {
                        if number > 0 {
                            number *= 10;
                        }
                        number += digit;
                    }
                    None => {
                        if number == 0 {
                            number += 1;
                        }
                        for _ in 0..number {
                            result.push(current_symbol);
                        }
                        number = 0;
                    }
                }
            }
            None => {
                break;
            }
        }
    }

    result
}
