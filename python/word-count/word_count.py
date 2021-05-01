def count_words(sentence):
    word_cnt = dict()
    for word in filter_and_split(sentence):
        if word in word_cnt:
            word_cnt[word] += 1
        else:
            word_cnt[word] = 1
    return word_cnt


def filter_and_split(sentence):
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
