// Rule 1
// If a word begins with a vowel, or starts with "xr" or "yt", add an "ay" sound to the end of the word.
// For example:
// "apple" -> "appleay" (starts with vowel)
// "xray" -> "xrayay" (starts with "xr")
// "yttria" -> "yttriaay" (starts with "yt")

// Rule 2
// If a word begins with one or more consonants, first move those consonants to the end of the word and then add an "ay" sound to the end of the word.
// For example:
// "pig" -> "igp" -> "igpay" (starts with single consonant)
// "chair" -> "airch" -> "airchay" (starts with multiple consonants)
// "thrush" -> "ushthr" -> "ushthray" (starts with multiple consonants)

// Rule 3
// If a word starts with zero or more consonants followed by "qu", first move those consonants (if any) and the "qu" part to the end of the word, and then add an "ay" sound to the end of the word.
// For example:
// "quick" -> "ickqu" -> "ickquay" (starts with "qu", no preceding consonants)
// "square" -> "aresqu" -> "aresquay" (starts with one consonant followed by "qu")

// Rule 4
// If a word starts with one or more consonants followed by "y", first move the consonants preceding the "y"to the end of the word, and then add an "ay" sound to the end of the word.
// Some examples:
// "my" -> "ym" -> "ymay" (starts with single consonant followed by "y")
// "rhythm" -> "ythmrh" -> "ythmrhay" (starts with multiple consonants followed by "y")

pub fn translate(input: &str) -> String {
    let vowel = ['e', 'u', 'i', 'o', 'a'];
    let consonants = [
        'q', 'w', 'r', 't', 'p', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'z', 'x', 'c', 'v', 'b',
        'n', 'm',
    ];

    let result_arr = input
        .split(' ')
        .map(|word| {
            let mut chars_arr: Vec<char> = word.chars().collect();

            // Rule 1
            if vowel.contains(&chars_arr[0]) || ["xr", "yt"].contains(&&word[0..2]) {
                return format!("{}ay", word);
            }

            let mut end: Vec<char> = Vec::new();

            // Rule 4
            if chars_arr[0] == 'y' {
                let y = chars_arr.remove(0);
                end.push(y);
            }

            // Rule 2
            loop {
                if chars_arr.len() > 1 {
                    // Rule 3
                    let first_two_chars: String = chars_arr.split_at(2).0.iter().collect();
                    if first_two_chars == "qu" {
                        let current_first = chars_arr.remove(0);
                        end.push(current_first);
                        let current_first = chars_arr.remove(0);
                        end.push(current_first);
                    }
                }

                if consonants.contains(&chars_arr[0]) {
                    let current_first = chars_arr.remove(0);
                    end.push(current_first);
                } else {
                    break;
                }
            }

            chars_arr.append(&mut end);
            let string: String = chars_arr.iter().collect();
            return format!("{}ay", string);
        })
        .collect::<Vec<String>>();

    result_arr.join(" ")
}
