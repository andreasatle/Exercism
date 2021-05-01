// Package triangle implements a function that determines
// if a triangle is equilateral, isosceles or scalene.
package triangle

import (
	"math"
)

// type Kind is a wrapper for int.
type Kind int

const (
	NaT Kind = iota // not a triangle
	Equ             // equilateral
	Iso             // isosceles
	Sca             // scalene
)

// KindFromSides returns the Kind of triangle.
func KindFromSides(a, b, c float64) Kind {
	// Check that arguments are valid (not NaN and Inf)
	if math.IsNaN(a*b*c) || math.IsInf(a*b*c, 0) {
		return NaT
	}

	// bubble-sort the arguments in ascending order
	// This reduces the number of checks to determine
	// the type of triangle.
	if a > b {
		a, b = b, a
	}
	if b > c {
		b, c = c, b
	}
	if a > b {
		a, b = b, a
	}

	// Check that smallest side positive
	if a <= 0 {
		return NaT
	}

	// Check triangle inequality
	if c > a+b {
		return NaT
	}

	// Check if all sides equal
	if a == b && a == c {
		return Equ
	}

	// Check is two sides are equal
	if a == b || b == c {
		return Iso
	}

	// All sides are different
	return Sca
}
