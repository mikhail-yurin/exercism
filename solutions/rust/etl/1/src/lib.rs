use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut result: BTreeMap<char, i32> = BTreeMap::new();

    for (key, symb_arr) in h {
        for symb in symb_arr {
            let new_key = *symb;

            for lc in new_key.to_lowercase() {
                result.insert(lc, *key);
            }
        }
    }

    result
}
