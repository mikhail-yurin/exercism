pub fn raindrops(n: u32) -> String {
    let mut drop = String::new();
    if n % 3 == 0 {
        drop.push_str("Pling");
    }
    if n % 5 == 0 {
        drop.push_str("Plang");
    }
    if n % 7 == 0 {
        drop.push_str("Plong");
    }
    if drop.is_empty() {
        n.to_string()
    } else {
        drop
    }
}
