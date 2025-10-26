pub struct PascalsTriangle(Vec<Vec<u32>>);

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut rows: Vec<Vec<u32>> = Vec::with_capacity(row_count as usize);
        for i in 0..row_count {
            let mut row: Vec<u32> = Vec::with_capacity((i + 1) as usize);

            if i > 0 {
                for j in 0..=i {
                    if j > 0 && j < i {
                        let prev_row = (i - 1) as usize;
                        let prev_col = (j - 1) as usize;
                        let current_col = j as usize;

                        row.push(rows[prev_row][prev_col] + rows[prev_row][current_col]);
                    } else {
                        row.push(1);
                    }
                }
            } else {
                row.push(1);
            }

            rows.push(row);
        }

        Self(rows)
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.0.clone()
    }
}
