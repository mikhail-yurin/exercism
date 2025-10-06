use std::collections::HashMap;

pub fn check(candidate: &str) -> bool {
    let mut map: HashMap<char, usize> = HashMap::new();

    for symb in candidate.chars() {
        for lc in symb.to_lowercase() {
            if let Some(count) = map.get(&lc) {
                map.insert(lc, count + 1);
            } else {
                map.insert(lc, 1);
            }
        }
    }

    for (key, count) in map {
        if ![' ', '-'].contains(&key) && count > 1 {
            return false;
        }
    }

    true
}
