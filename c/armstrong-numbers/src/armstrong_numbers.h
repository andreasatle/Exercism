// An Armstrong number is a number that is the sum of its own digits each raised
// to the power of the number of digits.
// 
// For example:
// 9 is an Armstrong number, because 9 = 9^1 = 9
// 10 is not an Armstrong number, because 10 != 1^2 + 0^2 = 1
// 153 is an Armstrong number, because: 153 = 1^3 + 5^3 + 3^3 = 1 + 125 + 27 = 153
// 154 is not an Armstrong number, because: 154 != 1^3 + 5^3 + 4^3 = 1 + 125 + 64 = 190

#ifndef ARMSTRONG_NUMBERS
#define ARMSTRONG_NUMBERS

#include <stdbool.h>

// integer pow-function (from stack overflow).
int ipow(int base, int exp);

// Count number of digits in a number.
int count_digits(int num);

// Checks whether a candidate is an Armstrong number.
bool is_armstrong_number(int candidate);

#endif
