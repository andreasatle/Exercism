from __future__ import division


class Rational:
    def __init__(self, numer: int, denom: int):
        if denom == 0:
            raise ValueError("Zero denominator is not allowed!")
        g = gcd(numer, denom)
        self.numer = numer / g
        self.denom = denom / g

    def __eq__(self, other):
        return self.numer == other.numer and self.denom == other.denom

    def __repr__(self):
        return '{}/{}'.format(self.numer, self.denom)

    def __add__(self, other):
        numer = self.numer*other.denom + self.denom*other.numer
        denom = self.denom*other.denom
        return Rational(numer, denom)

    def __sub__(self, other):
        numer = self.numer*other.denom - self.denom*other.numer
        denom = self.denom*other.denom
        return Rational(numer, denom)

    def __mul__(self, other):
        numer = self.numer*other.numer
        denom = self.denom*other.denom
        return Rational(numer, denom)

    def __truediv__(self, other):
        return Rational(self.numer*other.denom, self.denom*other.numer)

    def __abs__(self):
        return Rational(abs(self.numer), abs(self.denom))

    def __pow__(self, power: int):
        if power < 0:
            return Rational(self.denom**(-power), self.numer**(-power))
        return Rational(self.numer**power, self.denom**power)

    def __rpow__(self, base):
        return pow(base, self.numer/self.denom)


def gcd(a: int, b: int):
    if b == 0:
        return a
    return gcd(b, a % b)
