pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut result: Vec<String> = Vec::with_capacity(minefield.len());

    for (current_cell_line, line) in minefield.iter().enumerate() {
        let mut result_line = String::new();

        for (current_cell_column, cell_value) in line.chars().enumerate() {
            if cell_value == '*' {
                result_line.push(cell_value);
            } else {
                let cells_around: [(i8, i8); 8] = [
                    (-1, -1),
                    (-1, 0),
                    (-1, 1),
                    (0, -1),
                    (0, 1),
                    (1, -1),
                    (1, 0),
                    (1, 1),
                ];

                let mut count: i8 = 0;

                for cell in cells_around {
                    let line_index_checking = current_cell_line as i8 + cell.0;
                    if line_index_checking < 0
                        || (line_index_checking != 0
                            && line_index_checking > (minefield.len() - 1) as i8)
                    {
                        continue;
                    }

                    let column_index_checking = current_cell_column as i8 + cell.1;
                    if column_index_checking < 0 || column_index_checking >= (line.len()) as i8 {
                        continue;
                    }

                    let near_cell = minefield[line_index_checking as usize]
                        .chars()
                        .nth(column_index_checking as usize);

                    match near_cell {
                        None => {}
                        Some(val) => {
                            if val == '*' {
                                count += 1;
                            }
                        }
                    }
                }

                let smbl: char = if count == 0 {
                    ' '
                } else {
                    std::char::from_digit(count as u32, 10).unwrap()
                };
                result_line.push(smbl);
            }
        }

        result.push(result_line);
    }

    result
}
