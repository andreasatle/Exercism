'''Determine if a word or phrase is an isogram.

An isogram (also known as a "nonpattern word") is a word or phrase
without a repeating letter, however spaces and hyphens are allowed
to appear multiple times.

Examples of isograms:

lumberjacks
background
downstream
six-year-old

The word isograms, however, is not an isogram, because the s repeats.
'''


def is_isogram(string: str) -> bool:
    '''returns whether the input-string is an isogram.'''
    sorted_string = sorted(string.lower())
    for i in range(1, len(sorted_string)):
        if sorted_string[i] == sorted_string[i-1] and sorted_string[i].isalpha():
            return False
    return True
