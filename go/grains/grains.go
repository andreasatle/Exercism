// Package grains - Calculate the number of grains of wheat on a chessboard
// given that the number on each square doubles.
//
// There once was a wise servant who saved the life of a prince. The king
// promised to pay whatever the servant could dream up. Knowing that the king
// loved chess, the servant told the king he would like to have grains of
// wheat. One grain on the first square of a chess board, with the number of
// grains doubling on each successive square.
//
// There are 64 squares on a chessboard (where square 1 has one grain,
// square 2 has two grains, and so on).
//
package grains

import "errors"

// Square return the number of grains on input square.
func Square(input int) (int, error) {
	if input < 1 {
		return -1, errors.New("Non-positive input")
	}
	if input > 64 {
		return -1, errors.New("Input is too large")
	}

	return 1 << (input - 1), nil
}

// Total returns the total number of grains on the chessboard.
func Total() int {
	return (1 << 64) - 1
}
