//! Determine if a triangle is equilateral, isosceles, or scalene.
//! 
//! An equilateral triangle has all three sides the same length.
//! 
//! An isosceles triangle has at least two sides the same length. (It is sometimes specified as having exactly two sides the same length, but for the purposes of this exercise we'll say at least two.)
//! 
//! A scalene triangle has all sides of different lengths.

/// Contains the lengths of the sides of a triangle.
pub struct Triangle {
    sides: [u64;3],
}

impl Triangle {
    /// Returns on optional Triangle if the arguments are valid.
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        let mut sides = sides.clone();
        sides.sort();
        if sides[0] <= 0 || sides[2] > sides[0]+sides[1] {
            return None;
        }
        Some(Triangle{sides}) 
    }

    /// Returns whether the triangle is equilateral.
    pub fn is_equilateral(&self) -> bool {
        self.sides[0] == self.sides[2]
    }

    /// Returns whether the triangle is scalene.
    pub fn is_scalene(&self) -> bool {
        self.sides[0] < self.sides[1] && self.sides[1] < self.sides[2]
    }

    /// Returns whether the triangle is isosceles.
    pub fn is_isosceles(&self) -> bool {
        self.sides[0] == self.sides[1] || self.sides[1] == self.sides[2]
    }
}
