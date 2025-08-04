pub fn collatz(mut n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }

    let mut counter: u64 = 0;

    loop {
        if n == 1 {
            break;
        }

        if n % 2 == 0 {
            n /= 2;
        } else {
            n = 3 * n + 1
        }

        counter += 1;
    }

    Some(counter)
}
