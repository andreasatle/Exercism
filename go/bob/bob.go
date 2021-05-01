// Package bob has a routine Hay, that  responds depending
// on if you have capital letters, questions and some other criteria.
//
// Bob is a lackadaisical teenager. In conversation, his responses are very limited.
// Bob answers 'Sure.' if you ask him a question, such as "How are you?".
// He answers 'Whoa, chill out!' if you YELL AT HIM (in all capitals).
// He answers 'Calm down, I know what I'm doing!' if you yell a question at him.
// He says 'Fine. Be that way!' if you address him without actually saying anything.
// He answers 'Whatever.' to anything else.
// Bob's conversational partner is a purist when it comes to written communication and always follows normal rules regarding sentence punctuation in English.
package bob

import (
	"fmt"
	"unicode"
)

// Hey return phrases depending on the structure of the question.
func Hey(remark string) string {
	contents := NewContents(remark)
	fmt.Println(contents)
	// Check if no letters
	if !contents.lower && !contents.upper && !contents.number && !contents.question {
		return "Fine. Be that way!"
	}

	// Check if no lower + question
	if !contents.lower && contents.upper && contents.question {
		return "Calm down, I know what I'm doing!"
	}

	// Check if no lower
	if !contents.lower && contents.upper {
		return "Whoa, chill out!"
	}

	if contents.question {
		return "Sure."
	}

	return "Whatever."
}

type Contents struct {
	upper    bool
	lower    bool
	question bool
	number   bool
}

func NewContents(remark string) *Contents {
	contents := &Contents{}
	s := filter(remark)

	if len(s) > 0 && s[len(s)-1] == '?' {
		contents.question = true
	}

	for _, ch := range s {
		if unicode.IsLower(ch) {
			contents.lower = true
		}
		if unicode.IsUpper(ch) {
			contents.upper = true
		}
		if unicode.IsNumber(ch) {
			contents.number = true
		}
	}
	return contents
}

func filter(s string) string {
	out := []rune{}
	for _, r := range s {
		if unicode.IsLetter(r) || r == '?' || unicode.IsNumber(r) {
			out = append(out, r)
		}
	}
	return string(out)
}
