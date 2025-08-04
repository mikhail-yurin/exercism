pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut song: String = String::new();

    for num in (start_bottles - take_down + 1..=start_bottles).rev() {
        let (start, end) = match num {
            1 => ("One", "no"),
            2 => ("Two", "one"),
            3 => ("Three", "two"),
            4 => ("Four", "three"),
            5 => ("Five", "four"),
            6 => ("Six", "five"),
            7 => ("Seven", "six"),
            8 => ("Eight", "seven"),
            9 => ("Nine", "eight"),
            10 => ("Ten", "nine"),
            _ => ("", ""),
        };

        let verse = format!(
            "{} green bottle{} hanging on the wall,\n{} green bottle{} hanging on the wall,\nAnd if one green bottle should accidentally fall,\nThere'll be {} green bottle{} hanging on the wall.",
            start,
            if start == "One" {""} else {"s"},
            start,
            if start == "One" {""} else {"s"},
            end,
            if end == "one" {""} else {"s"},
        );

        song = format!("{}\n\n{}", song, verse);
    }

    song.trim().to_string()
}
