'''Convert a phrase to its acronym.

Techies love their TLA (Three Letter Acronyms)!

Help generate some jargon by writing a program that converts a long name
like Portable Network Graphics to its acronym (PNG).
'''


def abbreviate(words: str) -> str:
    '''Create and acronym from given words.'''
    splitted = words.replace('-', ' ').split()
    return "".join(["".join(filter(lambda x: x.isalpha(), list(word)))[0]
                    for word in splitted]).upper()


abbreviate('asg drih sehr seh')
