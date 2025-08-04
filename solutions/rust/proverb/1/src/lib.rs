pub fn build_proverb(list: &[&str]) -> String {
    let mut proverb = String::new();

    for (i, _) in list.iter().enumerate() {
        if i < list.len() - 1 {
            let line = format!("For want of a {} the {} was lost.\n", list[i], list[i + 1]);
            proverb.push_str(&line);
        } else {
            let line = format!("And all for the want of a {}.", list[0]);
            proverb.push_str(&line);
        }
    }

    proverb
}
