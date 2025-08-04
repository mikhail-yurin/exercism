pub fn abbreviate(phrase: &str) -> String {
    let uppercase_chars = "QWERTYUIOPASDFGHJKLZXCVBNM";

    let mut result: String = String::new();

    for word in phrase.split(" ") {
        let first = word.chars().next().unwrap().to_uppercase().next().unwrap();

        if word == word.to_uppercase() && uppercase_chars.contains(first) {
            result.push(first);
            continue;
        }

        for (i, char) in word.chars().enumerate() {
            if (i == 0 && uppercase_chars.contains(first))
                || uppercase_chars.contains(char)
                || (i > 0 && word.chars().nth(i - 1).unwrap() == '-')
            {
                result.push(char.to_uppercase().next().unwrap());
            }
        }
    }

    result
}
