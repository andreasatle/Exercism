//! Detect saddle points in a matrix.
//! 
//! So say you have a matrix like so:
//! 
//! ```comment
//!     1  2  3
//!   |---------
//! 1 | 9  8  7
//! 2 | 5  3  2     <--- saddle point at column 1, row 2, with value 5
//! 3 | 6  6  7
//! ```
//! It has a saddle point at column 1, row 2.
//! 
//! It's called a "saddle point" because it is greater than or equal to every
//! element in its row and less than or equal to every element in its column.
//! 
//! A matrix may have zero or more saddle points.
//! 
//! Your code should be able to provide the (possibly empty) list of all the
//! saddle points for any given matrix.
//! 
//! The matrix can have a different number of rows and columns (Non square).

use std::collections::HashSet;

/// Compute the saddle points in the input matrix.
pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    // Check for empty matrix.
    if input.len() == 0 || input[0].len() == 0 {
        return Vec::new();
    }
    let zero: usize = 0;
    let mut max_indices: HashSet<(usize, usize, u64)> = HashSet::new();
    let mut min_values = input[0].clone();

    // Loop over rows.
    for i in zero..input.len() {
        // Find max in current row.
        let max_val = input[i].iter().max().expect("Couldn't find max.");
        // Loop over columns.
        for j in zero..input[i].len() {
            // Find the indices of the max values in row.
            if input[i][j] == *max_val {
                max_indices.insert((i,j,input[i][j]));
            }
            // Find the min value in each column.
            if input[i][j] < min_values[j] {
                min_values[j] = input[i][j]
            }
        }
    }

    let mut output = Vec::new();
    for (i,j,max_val) in &max_indices {
        // Match max and min value
        if *max_val == min_values[*j] {
            output.push((*i,*j));
        }
    }
    output
}
