'''Implementation of a rational number class.
A rational number is defined as the quotient of two integers a and b, 
called the numerator and denominator, respectively, where b != 0.

The absolute value |r| of the rational number r = a/b is equal to |a|/|b|.

The sum of two rational numbers r1 = a1/b1 and r2 = a2/b2 is
r1 + r2 = a1/b1 + a2/b2 = (a1 * b2 + a2 * b1) / (b1 * b2).

The difference of two rational numbers r1 = a1/b1 and r2 = a2/b2 is
r1 - r2 = a1/b1 - a2/b2 = (a1 * b2 - a2 * b1) / (b1 * b2).

The product (multiplication) of two rational numbers r1 = a1/b1 and r2 = a2/b2 is
r1 * r2 = (a1 * a2) / (b1 * b2).

Dividing a rational number r1 = a1/b1 by another r2 = a2/b2 is 
r1 / r2 = (a1 * b2) / (a2 * b1) if a2 * b1 is not zero.

Exponentiation of a rational number r = a/b to a non-negative integer power n is 
r^n = (a^n)/(b^n).

Exponentiation of a rational number r = a/b to a negative integer power n is
r^n = (b^m)/(a^m), where m = |n|.

Exponentiation of a rational number r = a/b to a real (floating-point) number x
is the quotient (a^x)/(b^x), which is a real number.

Exponentiation of a real number x to a rational number
r = a/b is x^(a/b) = root(x^a, b), where root(p, q) is the qth root of p.

An exception ValueError is raised if the denominator is zero.
'''


class Rational:
    '''Implementation of a Rational number class.'''

    def __init__(self, numer: int, denom: int):
        if denom == 0:
            raise ValueError("Zero denominator is not allowed!")
        g = gcd(numer, denom)
        self.numer = numer / g
        self.denom = denom / g

    def __eq__(self, other: 'Rational') -> bool:
        return self.numer == other.numer and self.denom == other.denom

    def __repr__(self) -> str:
        return '{}/{}'.format(self.numer, self.denom)

    def __add__(self, other: 'Rational') -> 'Rational':
        numer = self.numer*other.denom + self.denom*other.numer
        denom = self.denom*other.denom
        return Rational(numer, denom)

    def __sub__(self, other: 'Rational') -> 'Rational':
        numer = self.numer*other.denom - self.denom*other.numer
        denom = self.denom*other.denom
        return Rational(numer, denom)

    def __mul__(self, other: 'Rational') -> 'Rational':
        numer = self.numer*other.numer
        denom = self.denom*other.denom
        return Rational(numer, denom)

    def __truediv__(self, other: 'Rational') -> 'Rational':
        return Rational(self.numer*other.denom, self.denom*other.numer)

    def __abs__(self) -> 'Rational':
        return Rational(abs(self.numer), abs(self.denom))

    def __pow__(self, power: int) -> 'Rational':
        if power < 0:
            return Rational(self.denom**(-power), self.numer**(-power))
        return Rational(self.numer**power, self.denom**power)

    def __rpow__(self, base: float) -> float:
        return pow(base, self.numer/self.denom)


def gcd(a: int, b: int) -> int:
    '''Computation of greatest common divisor.'''
    if b == 0:
        return a
    return gcd(b, a % b)
