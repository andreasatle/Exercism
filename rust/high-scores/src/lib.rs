use std::cmp;

#[derive(Debug)]
pub struct HighScores{
    arr: Vec<u32>
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        let hs = HighScores{arr: scores.to_vec()};
        hs
    }

    pub fn scores(&self) -> &[u32] {
        &self.arr[..]
    }

    pub fn latest(&self) -> Option<u32> {
        let n = self.arr.len();
        if n == 0 {
            return None;
        }
        Some(self.arr[n-1])
    }

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

    pub fn personal_top_three(&self) -> Vec<u32> {
        let n = self.arr.len();
        let mut sorted = self.arr.clone();
        sorted.sort();
        sorted.reverse();
        sorted.truncate(3);
        sorted
    }
}
