//! Given the size, return a square matrix of numbers in spiral order.
//! 
//! The matrix should be filled with natural numbers, starting from 1
//! in the top-left corner, increasing in an inward, clockwise spiral
//! order, like these examples:
//! 
//! # Examples
//! 
//! Spiral matrix of size 3
//! ```matrix
//! 1 2 3
//! 8 9 4
//! 7 6 5
//! ```
//! 
//! Spiral matrix of size 4
//! ```matrix
//!  1  2  3 4
//! 12 13 14 5
//! 11 16 15 6
//! 10  9  8 7
//! ```

/// Create a square spiral matrix of given size.
pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    if size == 0 {
        return Vec::new();
    }
    let mut matrix = vec![vec![0u32; size as usize]; size as usize];
    let mut lb_row = 0usize;
    let mut ub_row = (size-1) as usize;
    let mut lb_col = 0usize;
    let mut ub_col = (size-1) as usize;

    let mut cnt = 0;
    loop {
        for col in lb_col..=ub_col {
            cnt += 1;
            matrix[lb_row][col] = cnt;
        }
        lb_row += 1;
        if lb_row > ub_row {
            break;
        }

        for row in lb_row..=ub_row {
            cnt += 1;
            matrix[row][ub_col] = cnt;
        }
        ub_col -= 1;
        if lb_col > ub_col {
            break;
        }

        for col in (lb_col..=ub_col).rev() {
            cnt += 1;
            matrix[ub_row][col] = cnt;
        }
        ub_row -= 1;
        if lb_row > ub_row {
            break;
        }

        for row in (lb_row..=ub_row).rev() {
            cnt += 1;
            matrix[row][lb_col] = cnt;
        }
        lb_col += 1;
        if lb_col > ub_col {
            break;
        }

    }

    return matrix;
}
