'''Given a word, compute the scrabble score for that word.

Letter                           Value
A, E, I, O, U, L, N, R, S, T       1
D, G                               2
B, C, M, P                         3
F, H, V, W, Y                      4
K                                  5
J, X                               8
Q, Z                               10
'''


def score(word: str) -> int:
    ''' Returns the scrabble score for a given word.'''
    points = 0
    for ch in word.upper():
        if ch in 'AEIOULNRST':
            points += 1
        elif ch in 'DG':
            points += 2
        elif ch in 'BCMP':
            points += 3
        elif ch in 'FHVWY':
            points += 4
        elif ch in 'K':
            points += 5
        elif ch in 'JX':
            points += 8
        elif ch in 'QZ':
            points += 10
    return points
