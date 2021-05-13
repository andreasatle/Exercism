//! Given the position of two queens on a chess board,
//! indicate whether or not they are positioned so that
//! they can attack each other.
//! 
//! In the game of chess, a queen can attack pieces which
//! are on the same row, column, or diagonal.
//! 
//! A chessboard can be represented by an 8 by 8 array.
//! 
//! So if you're told the white queen is at (2, 3) and
//! the black queen at (5, 6), then you'd know you've got
//! a set-up like so:
//! 
//! ```comment
//! _ _ _ _ _ _ _ _
//! _ _ _ _ _ _ _ _
//! _ _ _ W _ _ _ _
//! _ _ _ _ _ _ _ _
//! _ _ _ _ _ _ _ _
//! _ _ _ _ _ _ B _
//! _ _ _ _ _ _ _ _
//! _ _ _ _ _ _ _ _
//! ```
//! 
//! You'd also be able to answer whether the queens can
//! attack each other. In this case, that answer would be
//! yes, they can, because both pieces share a diagonal.

/// Contains the position on the chess board.
#[derive(Debug)]
pub struct ChessPosition {
    row: i32,
    col: i32,
}

/// Contains the position for a queen.
#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    /// Create a new ChessPosition.
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if rank < 0 || rank >= 8 || file < 0 || file >= 8 {
            return None;
        }

        Some(ChessPosition{row: rank, col: file})
    }
}

impl Queen {
    /// Create a new Queen.
    pub fn new(position: ChessPosition) -> Self {
        Queen{position}
    }

    /// Decide if the queen can attack the other queen.
    pub fn can_attack(&self, other: &Queen) -> bool {
        let d_row = self.position.row - other.position.row;
        let d_col = self.position.col - other.position.col;
        d_row == 0 || d_col == 0 || d_row+d_col == 0 || d_row-d_col==0
    }
}
