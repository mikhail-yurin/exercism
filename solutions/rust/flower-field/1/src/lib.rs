fn is_flower(str: &str, col: usize) -> bool {
    str.chars().nth(col).unwrap_or(' ') == '*'
}

pub fn annotate(garden: &[&str]) -> Vec<String> {
    let mut field = vec![String::new(); garden.len()];

    for row_current in 0..garden.len() {
        for col_current in 0..garden[row_current].len() {
            if is_flower(garden[row_current], col_current) {
                field[row_current] = format!("{}*", field[row_current]);
            } else {
                let mut sum: usize = 0;

                let prev_col_exist = col_current > 0;
                let next_col_exist = col_current < garden[row_current].len() - 1;

                if let Some(row_prev) = row_current.checked_sub(1) {
                    if prev_col_exist && is_flower(garden[row_prev], col_current - 1) {
                        sum += 1;
                    }
                    if is_flower(garden[row_prev], col_current) {
                        sum += 1;
                    }
                    if next_col_exist && is_flower(garden[row_prev], col_current + 1) {
                        sum += 1;
                    }
                }

                if prev_col_exist && is_flower(garden[row_current], col_current - 1) {
                    sum += 1;
                }
                if next_col_exist && is_flower(garden[row_current], col_current + 1) {
                    sum += 1;
                }

                if row_current < garden.len() - 1 {
                    if prev_col_exist && is_flower(garden[row_current + 1], col_current - 1) {
                        sum += 1;
                    }
                    if is_flower(garden[row_current + 1], col_current) {
                        sum += 1;
                    }
                    if next_col_exist && is_flower(garden[row_current + 1], col_current + 1) {
                        sum += 1;
                    }
                }

                field[row_current] = format!(
                    "{}{}",
                    field[row_current],
                    if sum > 0 {
                        sum.to_string()
                    } else {
                        String::from(" ")
                    }
                );
            }
        }
    }

    field
}
