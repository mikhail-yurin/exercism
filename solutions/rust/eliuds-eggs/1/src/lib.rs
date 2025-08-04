pub fn egg_count(display_value: u32) -> usize {
    let binary_str = format!("{:b}", display_value);

    binary_str.chars().fold(
        0,
        |acc: usize, char| {
            if char == '1' {
                acc + 1
            } else {
                acc
            }
        },
    )
}
