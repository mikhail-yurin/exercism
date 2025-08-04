#[derive(Debug)]
pub struct HighScores {
    data: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        HighScores {
            data: scores.to_vec(),
        }
    }

    pub fn scores(&self) -> &[u32] {
        &self.data
    }

    pub fn latest(&self) -> Option<u32> {
        if self.data.is_empty() {
            return None;
        }
        Some(self.data[self.data.len() - 1])
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.data.iter().max().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut top_three: Vec<u32> = Vec::new();
        let mut all_scores = self.data.clone();

        let border = if all_scores.len() < 3 {
            all_scores.len()
        } else {
            3
        };

        for _ in 0..border {
            let max = *all_scores.iter().max().unwrap();
            top_three.push(max);
            let max_position = all_scores.iter().position(|&x| x == max).unwrap();
            all_scores.remove(max_position);
        }

        top_three
    }
}
