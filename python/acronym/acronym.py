# Remark: This is built on top of a previous exercise: word-count and could be rather heavily optimized.


def abbreviate(words):
    # Make upper case and create list of chars
    chars = [ch for ch in words.upper()]

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

    # Extract first letter
    s1 = ""
    for ch in s.split():
        s1 += ch[0]
    return s1
