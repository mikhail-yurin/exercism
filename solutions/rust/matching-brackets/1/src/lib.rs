pub fn brackets_are_balanced(string: &str) -> bool {
    let open_chars = ['(', '[', '{'];
    let close_chars = [')', ']', '}'];

    let mut new_string = String::new();

    for char in string.chars() {
        if open_chars.contains(&char) || close_chars.contains(&char) {
            new_string.push(char);
        }
    }

    loop {
        new_string = new_string
            .replace("()", "")
            .replace("[]", "")
            .replace("{}", "");

        if !new_string.contains("()") && !new_string.contains("[]") && !new_string.contains("{}") {
            break;
        }
    }

    new_string.is_empty()
}
