def distance(strand_a, strand_b):
    if len(strand_a) != len(strand_b):
        raise ValueError("Different lengths of input.")

    cnt = 0
    for i in range(len(strand_a)):
        if strand_a[i] != strand_b[i]:
            cnt += 1
    return cnt
