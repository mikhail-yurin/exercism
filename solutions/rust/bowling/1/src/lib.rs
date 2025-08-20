#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    throws: [u8; 21],
    throws_done: usize,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            throws: [0; 21],
            throws_done: 0,
        }
    }

    pub fn roll(&mut self, pins: u8) -> Result<(), Error> {
        if self.throws_done >= 21 {
            return Err(Error::GameComplete);
        }

        let is_frame_end =
            self.throws_done == 20 || self.throws_done % 2 != 0 && self.throws_done != 19;

        if pins > 10
            || self.throws_done < 19
                && is_frame_end
                && pins + self.throws[self.throws_done - 1] > 10
            || self.throws_done == 20
                && self.throws[18] == 10
                && self.throws[19] < 10
                && self.throws[19] + pins > 10
        {
            return Err(Error::NotEnoughPinsLeft);
        }

        self.throws[self.throws_done] = pins;
        self.throws_done += 1;

        if pins == 10 && self.throws_done < 18 {
            self.throws_done += 1;
        }

        // Final frame
        // no bonus
        if self.throws_done == 20 && self.throws[18] + self.throws[19] < 10 {
            self.throws_done += 1;
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        let has_bonuse_in_last_frame = self.throws[18] + self.throws[19] >= 10;

        if has_bonuse_in_last_frame && self.throws_done < 21
            || !has_bonuse_in_last_frame && self.throws_done < 20
        {
            return None;
        }

        let mut total_score: u16 = 0;

        for (throw, pins) in self.throws.iter().enumerate() {
            let is_frame_start = throw < 18 && throw % 2 == 0;

            total_score += *pins as u16;

            // strike
            if is_frame_start && *pins == 10 {
                if self.throws[throw + 2] == 10 && throw < 16 {
                    total_score += (self.throws[throw + 2] + self.throws[throw + 4]) as u16;
                } else {
                    total_score += (self.throws[throw + 2] + self.throws[throw + 3]) as u16;
                }
            }

            // spare
            if is_frame_start && *pins < 10 && self.throws[throw] + self.throws[throw + 1] == 10 {
                total_score += (self.throws[throw + 2]) as u16;
            }
        }

        Some(total_score)
    }
}
