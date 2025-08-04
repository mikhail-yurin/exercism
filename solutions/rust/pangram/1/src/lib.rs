/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    let mut is_a_panagram = true;

    for char in alphabet.chars() {
        if !sentence.to_lowercase().contains(char) {
            is_a_panagram = false;
        }
    }

    is_a_panagram
}
