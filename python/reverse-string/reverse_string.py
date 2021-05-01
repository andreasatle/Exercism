def reverse(text: str):
    chars = list(text)
    for i in range(len(text)//2):
        chars[i], chars[-i-1] = chars[-i-1], chars[i]
    return "".join(chars)
