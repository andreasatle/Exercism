//! Calculate the number of grains of wheat on a chessboard given that the number on each square doubles.
//! 
//! There once was a wise servant who saved the life of a prince. The king promised to pay whatever the servant could dream up. Knowing that the king loved chess, the servant told the king he would like to have grains of wheat. One grain on the first square of a chess board, with the number of grains doubling on each successive square.
//! 
//! There are 64 squares on a chessboard (where square 1 has one grain, square 2 has two grains, and so on).
//! 
//! Write code that shows:
//! How many grains were on a given square, and the total number of grains on the chessboard.

/// Computes the number of grains on square s.
pub fn square(s: u32) -> u64 {
    if s < 1 || s > 64 {
        panic!("Square must be between 1 and 64");
    }
    1 << (s - 1)
}

/// Computes the total number of grains on a chess board.
pub fn total() -> u64 {
    (1..=64).fold(0, |a, b| a + square(b))
}
