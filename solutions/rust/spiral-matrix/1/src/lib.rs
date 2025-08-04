#[derive(PartialEq)]
enum Direction {
    Right,
    Left,
    Down,
    Up,
}

pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut result: Vec<Vec<u32>> = Vec::with_capacity(size as usize);
    for _ in 0..size {
        let row: Vec<u32> = vec![0; size as usize];
        result.push(row);
    }

    let mut current_direction: Direction = Direction::Right;
    let mut current_row: usize = 0;
    let mut current_column: usize = 0;

    for step in 1..=size * size {
        println!("[{}, {}] = {}", current_row + 1, current_column + 1, step);

        result[current_row][current_column] = step;

        if current_direction == Direction::Right
            && (result[current_row].get(current_column + 1).is_none()
                || result[current_row][current_column + 1] != 0)
        {
            current_direction = Direction::Down;
        }

        if current_direction == Direction::Down
            && (result.get(current_row + 1).is_none()
                || result[current_row + 1][current_column] != 0)
        {
            current_direction = Direction::Left;
        }

        if current_direction == Direction::Left
            && (current_column == 0 || result[current_row][current_column - 1] != 0)
        {
            current_direction = Direction::Up;
        }

        if current_direction == Direction::Up
            && (current_row == 0 || result[current_row - 1][current_column] != 0)
        {
            current_direction = Direction::Right;
        }

        match current_direction {
            Direction::Right => current_column += 1,
            Direction::Down => current_row += 1,
            Direction::Left => current_column -= 1,
            Direction::Up => current_row -= 1,
        }
    }

    result
}
