//! Manage a game player's High Score list.
//! 
//! Your task is to build a high-score component of the classic Frogger game, 
//! one of the highest selling and addictive games of all time, and a classic 
//! of the arcade era. Your task is to write methods that return the highest 
//! score from the list, the last added score and the three highest scores.

/// Contains the scores from the game Frogger.
#[derive(Debug)]
pub struct HighScores{
    arr: Vec<u32>
}

impl HighScores {
    /// Constructor for the struct HighScores.
    pub fn new(scores: &[u32]) -> Self {
        let hs = HighScores{arr: scores.to_vec()};
        hs
    }

    /// Returns the scores.
    pub fn scores(&self) -> &[u32] {
        &self.arr[..]
    }

    /// Returns an optional with the latest score. If no scores exist, None is returned.
    pub fn latest(&self) -> Option<u32> {
        let n = self.arr.len();
        if n == 0 {
            return None;
        }
        Some(self.arr[n-1])
    }

    /// Returns an optional with the personal best score. If no scores exist, None is returned.
    pub fn personal_best(&self) -> Option<u32> {
        let n = self.arr.len();
        if n == 0 {
            return None;
        }
        let mut best_score = 0;
        for i in 0..n {
            if self.arr[i] > best_score {
                best_score = self.arr[i];
            }
        }
        Some(best_score)
    }

    /// Returns the personal top three scores.
    pub fn personal_top_three(&self) -> Vec<u32> {
        let n = self.arr.len();
        if n < 3 {
            if n <= 1 || (n == 2 && self.arr[0] >= self.arr[1]) {
                return self.arr.clone();
            }
            return vec![self.arr[1], self.arr[0]];
        }

        let mut t = vec![0,0,0];
        let mut tmp;
        for s in self.arr.iter() {
            if *s > t[2] {
                t[2] = *s;
                if t[1] < t[2] {
                    tmp = t[1]; t[1] = t[2]; t[2] = tmp;
                }
                if t[0] < t[1] {
                    tmp = t[0]; t[0] = t[1]; t[1] = tmp;
                }
                if t[1] < t[2] {
                    tmp = t[1]; t[1] = t[2]; t[2] = tmp;
                }
            }
        }
        t
    }
}
