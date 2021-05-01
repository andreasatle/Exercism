def steps(number: int):
    if number <= 0:
        raise ValueError(f"Argument: number = {number} is too small.")

    cnt = 0
    while number != 1:
        number = number >> 1 if number & 1 == 0 else 3*number+1
        cnt += 1
    return cnt
