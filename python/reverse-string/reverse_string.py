'''Reverse a string

For example: input: "cool" output: "looc"
'''


def reverse(text: str) -> str:
    '''Return the string in reverse.'''
    chars = list(text)
    for i in range(len(text)//2):
        chars[i], chars[-i-1] = chars[-i-1], chars[i]
    return "".join(chars)
