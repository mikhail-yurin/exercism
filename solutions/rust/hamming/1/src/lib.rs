/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
        return None;
    }

    let mut distance: usize = 0;

    for i in 0..s1.len() {
        if s1.get(i..i + 1) != s2.get(i..i + 1) {
            distance += 1;
        }
    }

    Some(distance)
}
