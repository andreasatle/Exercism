// Package darts computes the score of a dartgame
// centered at the origin with scores:
//  r <= 1:  10pts
//  r <= 5:  5pts
//  r <= 10: 1pt
//  r >  10: 0pt
package darts

// Score returs the score of one dart on a dart-board.
func Score(x, y float64) int {
	r2 := x*x + y*y
	if r2 <= 1.0 {
		return 10
	}
	if r2 <= 25.0 {
		return 5
	}
	if r2 <= 100.0 {
		return 1
	}
	return 0
}
