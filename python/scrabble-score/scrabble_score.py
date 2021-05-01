
def score(word):
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
