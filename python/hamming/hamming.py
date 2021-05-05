'''Calculate the Hamming Distance between two DNA strands.

Your body is made up of cells that contain DNA. Those cells 
regularly wear out and need replacing, which they achieve by
dividing into daughter cells. In fact, the average human body
experiences about 10 quadrillion cell divisions in a lifetime!

When cells divide, their DNA replicates too. Sometimes during
this process mistakes happen and single pieces of DNA get encoded
with the incorrect information. If we compare two strands of DNA
and count the differences between them we can see how many mistakes
occurred. This is known as the "Hamming Distance".

Exceptions: When two strands are of different length,
a ValueError is raised.
'''


def distance(strand_a: str, strand_b: str) -> int:
    '''returns the Hamming distance of to strands.'''
    if len(strand_a) != len(strand_b):
        raise ValueError("Different lengths of input.")

    cnt = 0
    for i in range(len(strand_a)):
        if strand_a[i] != strand_b[i]:
            cnt += 1
    return cnt
