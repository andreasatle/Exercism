# Follow derivation from brilliant.org
# Sn = sum(k, 1, n), Tn = sum(k ^ 2, 1, n)
# Start with Sn: sum((k-1) ^ 2, 1, n) = sum(k ^ 2-2*k+1, 1, n)
# Rearrange: n ^ 2 = sum(k ^ 2-(k-1) ^ 2, 1, n) = 2*Sn-n
# Finally: Sn = n*(n+1)/2
# Now Tn: sum(k ^ 3-(k-1) ^ 3, 1, n) = 3*Tn-3*Sn+n.
# Finally: Tn = n*(2*n+1)*(n+1)/6

def square_of_sum(n: int):
    return n * n * (n + 1) * (n + 1) // 4


def sum_of_squares(n: int):
    return n * (2 * n + 1) * (n + 1) // 6


def difference_of_squares(n: int):
    return square_of_sum(n) - sum_of_squares(n)
