#include "armstrong_numbers.h"

// Checks whether a candidate is an Armstrong number.
bool is_armstrong_number(int candidate) {
    // Compute the exponent (number of digits of candidate).
    int exp = count_digits(candidate);

    int armstrong_num = 0;
    for (int num = candidate; num > 0; num /= 10) {
        armstrong_num += ipow(num%10, exp);
    }
    return armstrong_num == candidate;

}

// integer pow-function (from stack overflow).
int ipow(int base, int exp)
{
    int result = 1;
    for (;;)
    {
        if (exp & 1)
            result *= base;
        exp >>= 1;
        if (!exp)
            break;
        base *= base;
    }

    return result;
}

// Count number of digits in a number.
int count_digits(int num) {
    int cnt = 0;
    while (num > 0) {
        num /= 10;
        cnt++;
    }
    return cnt;
}
