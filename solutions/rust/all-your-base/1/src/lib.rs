#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base < 2 {
        return Err(Error::InvalidInputBase);
    }

    if to_base < 2 {
        return Err(Error::InvalidOutputBase);
    }

    let mut result: Vec<u32> = Vec::new();

    if number.is_empty() {
        result.push(0);
        return Ok(result);
    }

    for digit in number.iter() {
        if *digit >= from_base {
            return Err(Error::InvalidDigit(*digit));
        }
    }

    let mut a_number: Vec<u32> = Vec::new();

    if from_base != 10 {
        let mut sum: u32 = 0;

        for (i, digit) in number.iter().rev().enumerate() {
            sum += digit * from_base.pow(i as u32);
        }

        for char in sum.to_string().chars() {
            match char.to_digit(10) {
                None => return Err(Error::InvalidDigit(char.to_digit(10).unwrap())),
                Some(digit) => {
                    a_number.push(digit);
                }
            };
        }
    } else {
        a_number = number.to_vec();
    }

    if to_base == 10 {
        return Ok(a_number);
    } else {
        let mut division_result: u32 = a_number.iter().fold(0, |acc, &digit| acc * 10 + digit);

        loop {
            let remain = division_result % to_base;
            division_result /= to_base;

            result.push(remain);

            if division_result == 0 {
                break;
            }
        }
        result.reverse();
    }

    Ok(result)
}
