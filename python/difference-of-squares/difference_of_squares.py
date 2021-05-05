'''Difference between the square of the sum and the sum of the squares

Find the difference between the square of the sum and the sum 
of the squares of the first N natural numbers.

The square of the sum of the first ten natural numbers is 
(1 + 2 + ... + 10)² = 55² = 3025.

The sum of the squares of the first ten natural numbers is 
1² + 2² + ... + 10² = 385.

Hence the difference between the square of the sum of the first 
ten natural numbers and the sum of the squares of the first ten 
natural numbers is 3025 - 385 = 2640.

Following a derivation from brilliant.org
Sn = sum(k, 1, n), Tn = sum(k ^ 2, 1, n)
Start with Sn: sum((k-1) ^ 2, 1, n) = sum(k ^ 2-2*k+1, 1, n)
Rearrange: n ^ 2 = sum(k ^ 2-(k-1) ^ 2, 1, n) = 2*Sn-n
Finally: Sn = n*(n+1)/2
Now Tn: sum(k ^ 3-(k-1) ^ 3, 1, n) = 3*Tn-3*Sn+n.
Finally: Tn = n*(2*n+1)*(n+1)/6
'''


def square_of_sum(n: int) -> int:
    '''(1 + 2 + ... + n)^2.'''
    return n * n * (n + 1) * (n + 1) // 4


def sum_of_squares(n: int) -> int:
    '''(1^2 + 2^2 + ... + n^2).'''
    return n * (2 * n + 1) * (n + 1) // 6


def difference_of_squares(n: int) -> int:
    '''(1 + 2 + ... + n)^2 - (1^2 + 2^2 + ... + n^2).'''
    return square_of_sum(n) - sum_of_squares(n)
