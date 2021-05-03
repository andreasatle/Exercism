pub struct Triangle {
    sides: [u64;3],
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        let mut sides = sides.clone();
        sides.sort();
        if sides[0] <= 0 || sides[2] > sides[0]+sides[1] {
            return None;
        }
        Some(Triangle{sides}) 
    }

    pub fn is_equilateral(&self) -> bool {
        self.sides[0] == self.sides[2]
    }

    pub fn is_scalene(&self) -> bool {
        self.sides[0] < self.sides[1] && self.sides[1] < self.sides[2]
    }

    pub fn is_isosceles(&self) -> bool {
        self.sides[0] == self.sides[1] || self.sides[1] == self.sides[2]
    }
}
