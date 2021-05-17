// Package raindrops - Your task is to convert a number into a string
// that contains raindrop sounds corresponding to certain potential factors.
// A factor is a number that evenly divides into another number, leaving no
// remainder. The simplest way to test if a one number is a factor of
// another is to use the modulo operation.
//
// The rules of raindrops are that if a given number:
//
// has 3 as a factor, add 'Pling' to the result.
// has 5 as a factor, add 'Plang' to the result.
// has 7 as a factor, add 'Plong' to the result.
// does not have any of 3, 5, or 7 as a factor, the result should be
// the digits of the number.
package raindrops

import "strconv"

// Convert outputs a string containing "Pling", "Plang", "Plong" or the number.
func Convert(num int) string {
	out := ""
	if num%3 == 0 {
		out = "Pling"
	}
	if num%5 == 0 {
		out = out + "Plang"
	}
	if num%7 == 0 {
		out = out + "Plong"
	}
	if len(out) == 0 {
		out = strconv.Itoa(num)
	}

	return out
}
