// Package reverse contains a routine to reverse a string.
//
// For example: input: "cool" output: "looc"
package reverse

// Reverse returns a string is reverse order.
func Reverse(s string) string {
	sr := []rune(s)
	for i, j := 0, len(sr)-1; i < j; i, j = i+1, j-1 {
		sr[i], sr[j] = sr[j], sr[i]
	}
	return string(sr)
}
