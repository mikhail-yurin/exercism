pub fn verse(n: u32) -> String {
    let words: (String, String) = match n {
        0 => (
            "No more bottles".to_string(),
            "Go to the store and buy some more, 99 bottles".to_string(),
        ),
        1 => (
            format!("{n} bottle"),
            "Take it down and pass it around, no more bottles".to_string(),
        ),
        2 => (
            format!("{n} bottles"),
            format!("Take one down and pass it around, {} bottle", n - 1),
        ),
        _ => (
            format!("{n} bottles"),
            format!("Take one down and pass it around, {} bottles", n - 1),
        ),
    };

    format!(
        "{} of beer on the wall, {} of beer.\n{} of beer on the wall.",
        words.0,
        words.0.to_lowercase(),
        words.1,
    )
}

pub fn sing(start: u32, end: u32) -> String {
    let mut song: Vec<String> = vec![];

    let mut i = start;

    loop {
        song.push(verse(i));

        if i == end {
            break;
        } else if i == 0 {
            i = 99;
        } else {
            i -= 1;
        }
    }

    song.join("\n\n")
}
