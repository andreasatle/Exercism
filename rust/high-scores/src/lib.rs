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
        if n < 3 {
            if n <= 1 || (n == 2 && self.arr[0] >= self.arr[1]) {
                return self.arr.clone();
            }
            return vec![self.arr[1], self.arr[0]];
        }

        let mut t = vec![0,0,0];
        let mut tmp = 0;
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
