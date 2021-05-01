def is_isogram(string):

    sorted_string = sorted(string.lower())
    for i in range(1, len(sorted_string)):
        if sorted_string[i] == sorted_string[i-1] and sorted_string[i].isalpha():
            return False
    return True
