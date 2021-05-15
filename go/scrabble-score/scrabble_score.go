// Package scrablle contains a routine that computes the scabble score of a string.
package scrabble

var table map[rune]int

func init() {
	table = map[rune]int{}
	for _, ch := range "AEIOULNRSTaeioulnrst" {
		table[ch] = 1
	}
	for _, ch := range "DGdg" {
		table[ch] = 2
	}
	for _, ch := range "BCMPbcmp" {
		table[ch] = 3
	}
	for _, ch := range "FHVWYfhvwy" {
		table[ch] = 4
	}
	for _, ch := range "Kk" {
		table[ch] = 5
	}
	for _, ch := range "JXjx" {
		table[ch] = 8
	}
	for _, ch := range "QZqz" {
		table[ch] = 10
	}
}

// Given a word, compute the Scrabble score for that word.
func Score(s string) int {
	score := 0
	for _, ch := range s {
		if sc, ok := table[ch]; ok {
			score += sc
		}
	}
	return score
}
