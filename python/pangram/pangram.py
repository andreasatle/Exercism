'''Determine if a sentence is a pangram.

A pangram (Greek: παν γράμμα, pan gramma, "every letter")
is a sentence using every letter of the alphabet at least once.
The best known English pangram is:

The quick brown fox jumps over the lazy dog.

The alphabet used consists of ASCII letters a to z, inclusive,
and is case insensitive. Input will not contain non-ASCII symbols.
'''


def is_pangram(sentence: str):
    '''Returns whether the sentence is a pangram.'''
    letter_set = {}
    for c in sentence.lower():
        if c.isalpha():
            letter_set[c] = None
    return len(letter_set) == 26
