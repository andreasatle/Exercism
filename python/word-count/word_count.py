'''Given a phrase, count the occurrences of each word in that phrase.

For the purposes of this exercise you can expect that a word will always be one of:

A number composed of one or more ASCII digits (ie "0" or "1234") OR
A simple word composed of one or more ASCII letters (ie "a" or "they") OR
A contraction of two simple words joined by a single apostrophe (ie "it's" or "they're")
When counting words you can assume the following rules:

The count is case insensitive (ie "You", "you", and "YOU" are 3 uses of the same word)
The count is unordered; the tests will ignore how words and counts are ordered
Other than the apostrophe in a contraction all forms of punctuation are ignored
The words can be separated by any form of whitespace (ie "\t", "\n", " ")
'''


def count_words(sentence: str) -> dict[str, int]:
    '''Returns a dictionary with the case-insensitive word-counts.'''
    word_cnt = dict()
    for word in _filter_and_split(sentence):
        if word in word_cnt:
            word_cnt[word] += 1
        else:
            word_cnt[word] = 1
    return word_cnt


def _filter_and_split(sentence: str) -> list[str]:
    '''Filter and split the sentence into a list of words.'''
    # Make lower case and create list of chars
    chars = [ch for ch in sentence.lower()]

    n = len(chars)

    # loop over chars and replace non-alphanumerical chars with space.
    # Except if for an ' surrounded by letters.
    for i in range(n):

        if chars[i].isalnum():
            continue
        if i > 0 and i < n-1 and chars[i] == "'" and chars[i-1].isalpha() and chars[i+1].isalpha():
            continue
        chars[i] = ' '

    # join list to a string
    s = ""
    for ch in chars:
        s += ch
    return s.split()
