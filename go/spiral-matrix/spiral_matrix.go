// Package spiral matrix assembles a square matrix in a spiral manner.
//
// Given the size, return a square matrix of numbers in spiral order.
//
// The matrix should be filled with natural numbers, starting from 1 in the top-left corner, increasing in an inward, clockwise spiral order, like these examples:
//
// Spiral matrix of size 3
//  1 2 3
//  8 9 4
//  7 6 5
// Spiral matrix of size 4
//   1  2  3 4
//  12 13 14 5
//  11 16 15 6
//  10  9  8 7
package spiralmatrix

// SpiralMatrix returns a square matrix, assembles in a spiral pattern.
func SpiralMatrix(n int) [][]int {
	// Special case for empty matrix
	if n <= 0 {
		return [][]int{}
	}

	mat := make([][]int, n)
	for i := 0; i < n; i++ {
		mat[i] = make([]int, n)
	}
	cnt := 1
	imin, imax, jmin, jmax := 0, n-1, 0, n-1
	for {
		for j := jmin; j <= jmax; j++ {
			mat[imin][j] = cnt
			cnt++
		}
		imin++
		if imin > imax {
			return mat
		}
		for i := imin; i <= imax; i++ {
			mat[i][jmax] = cnt
			cnt++
		}
		jmax--
		if jmin > jmax {
			return mat
		}
		for j := jmax; j >= jmin; j-- {
			mat[imax][j] = cnt
			cnt++
		}
		imax--
		if imin > imax {
			return mat
		}
		for i := imax; i >= imin; i-- {
			mat[i][jmin] = cnt
			cnt++
		}
		jmin++
		if jmin > jmax {
			return mat
		}
	}
	return mat
}
