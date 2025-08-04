pub fn factors(n: u64) -> Vec<u64> {
    let mut f: Vec<u64> = Vec::new();
    let mut delitel: u64 = 2;
    let mut delimoe: u64 = n;
    loop {
        if delimoe < delitel {
            break;
        } else if delimoe == delitel {
            f.push(delitel);
            break;
        } else if delimoe % delitel == 0 {
            f.push(delitel);
            delimoe /= delitel;
        } else {
            delitel += 1;
        }
    }
    f
}
