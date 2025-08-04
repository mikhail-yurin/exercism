pub fn square(s: u32) -> u128 {
    if s == 1 {
        1
    } else if s > 64 {
        panic!()
    } else {
        2_u128.pow(s - 1)
    }
}

pub fn total() -> u128 {
    let mut sum: u128 = 0;

    for i in 1..=64 {
        sum += square(i)
    }

    sum
}
