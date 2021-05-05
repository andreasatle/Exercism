'''Manage a game player's High Score list.

Your task is to build a high-score component of the classic
Frogger game, one of the highest selling and addictive games
of all time, and a classic of the arcade era. Your task is to
write methods that return the highest score from the list, the
last added score and the three highest scores.
'''


def latest(scores: list[int]) -> int:
    '''returns the latest score from the Frogger game.'''
    return scores[-1]


def personal_best(scores: list[int]) -> int:
    '''returns the personal best score from the Frogger game.'''
    return max(scores)


def personal_top_three(scores: list[int]) -> list[int]:
    '''returns the top three scores from the Frogger game.'''
    # Special cases
    if len(scores) == 1:
        return scores
    if len(scores) == 2:
        return [max(scores), min(scores)]

    # Normal case when len(scores) >= 3
    top_three = scores[:3]

    # Sort in ascending order
    top_three.sort()
    for score in scores[3:]:
        # Hardcoded sort, with too much indentation.
        if score > top_three[0]:
            top_three[0] = score
            if top_three[0] > top_three[1]:
                top_three[0], top_three[1] = top_three[1], top_three[0]
                if top_three[1] > top_three[2]:
                    top_three[1], top_three[2] = top_three[2], top_three[1]
    # Reverse result to descending order
    top_three.reverse()
    return top_three
