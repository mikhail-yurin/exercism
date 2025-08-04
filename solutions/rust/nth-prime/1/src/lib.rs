pub fn nth(n: u32) -> u32 {
    if n == 0 {
        return 2;
    }
    let mut count = 0;
    let mut candidate: u32 = 2;

    while count < n {
        candidate += 1;
        if is_prime(candidate) {
            count += 1;
        }
    }

    candidate
}

fn is_prime(num: u32) -> bool {
    if num < 2 {
        return false;
    }
    for i in 2..=((num as f64).sqrt() as u32) {
        if num % i == 0 {
            return false;
        }
    }
    true
}
