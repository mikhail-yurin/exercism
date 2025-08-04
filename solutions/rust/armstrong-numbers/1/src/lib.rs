pub fn is_armstrong_number(num: u32) -> bool {
    let num_string = num.to_string();
    let exponent = num_string.len() as u32;
    let mut sum: u32 = 0;

    for char in num_string.chars() {
        let digit: Result<u32, _> = char.to_string().parse();
        match digit {
            Err(_) => {
                println!("Symbol is not a digit");
                std::process::exit(1);
            }
            Ok(val) => {
                sum += val.pow(exponent);
            }
        }
    }

    num == sum
}
