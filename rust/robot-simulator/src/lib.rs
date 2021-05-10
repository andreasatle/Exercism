//! Write a robot simulator.
//! 
//! A robot factory's test facility needs a program to verify robot movements.
//! 
//! The robots have three possible movements:
//! 
//! * turn right
//! * turn left
//! * advance
//! 
//! Robots are placed on a hypothetical infinite grid, facing a particular direction
//! (north, east, south, or west) at a set of {x,y} coordinates, e.g., {3,8}, with
//! coordinates increasing to the north and east.
//! 
//! The robot then receives a number of instructions, at which point the testing
//! facility verifies the robot's new position, and in which direction it is pointing.
//! 
//! The letter-string "RAALAL" means:
//! * Turn right
//! * Advance twice
//! * Turn left
//! * Advance once
//! * Turn left yet again
//! 
//! Say a robot starts at {7, 3} facing north. Then running this stream of
//! instructions should leave it at {9, 4} facing west.

#[derive(PartialEq, Debug)]
/// Directions are aligned with the axes, i.e. N, E, S, W.
pub enum Direction {
    North,
    East,
    South,
    West,
}

/// Contains a coordinate (x, y) and a Direction.
pub struct Robot {
    x: i32,
    y: i32,
    d: Direction,
}

impl Robot {
    /// Constructor of the struct.
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot{x, y, d}
    }

    /// Modify the state of the robot turning right.
    pub fn turn_right(self) -> Self {
        Robot{
            x: self.x,
            y: self.y,
            d: match self.d {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
            },
        }
    }

    /// Modify the state of the robot turning left.
    pub fn turn_left(self) -> Self {
        Robot{
            x: self.x,
            y: self.y,
            d: match self.d {
                Direction::North => Direction::West,
                Direction::East => Direction::North,
                Direction::South => Direction::East,
                Direction::West => Direction::South,
            },
        }
    }

    /// Advance along the current direction.
    pub fn advance(self) -> Self {
        let mut x = self.x;
        let mut y = self.y;
        
        match self.d {
            Direction::North => {y += 1;},
            Direction::East => {x += 1;},
            Direction::South => {y -= 1;},
            Direction::West => {x -= 1;},
        }
        Robot{x, y, d: self.d}
    }

    /// Read and execute some instructions.
    /// * R: turn right,
    /// * L: turn left,
    /// * A: Advance forward one step.
    pub fn instructions(self, instructions: &str) -> Self {
        let mut next = self;
        for instruction in instructions.chars() {
            next = match instruction {
                'R' => next.turn_right(),
                'L' => next.turn_left(),
                'A' => next.advance(),
                _ => next,
            }
        }
        next
    }

    /// Getter for the position (x, y).
    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    /// Getter for the direction.
    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
