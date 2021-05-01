def is_pangram(sentence):
    letter_set = {}
    for c in sentence.lower():
        if c.isalpha():
            letter_set[c] = None
    return len(letter_set) == 26
