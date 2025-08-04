use regex::Regex;

pub fn reply(message: &str) -> &str {
    let trimmed = message.trim();

    let re = Regex::new(r"[a-zA-Z]").unwrap();
    let contain_letters = re.is_match(message);

    println!("contain_letters: {}", contain_letters);

    if trimmed.is_empty() {
        return "Fine. Be that way!";
    }
    if contain_letters && trimmed.ends_with("?") && trimmed == trimmed.to_uppercase() {
        return "Calm down, I know what I'm doing!";
    }
    if trimmed.ends_with("?") {
        return "Sure.";
    }
    if contain_letters && trimmed == trimmed.to_uppercase() {
        return "Whoa, chill out!";
    }

    "Whatever."
}
