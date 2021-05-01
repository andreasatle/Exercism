def latest(scores):
    return scores[-1]


def personal_best(scores):
    return max(scores)


def personal_top_three(scores):
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
