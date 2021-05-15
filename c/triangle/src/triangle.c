#include "triangle.h"

bool is_equilateral(triangle_t t) {
    return is_valid(t) && t.a == t.b && t.a == t.c;
}

bool is_isosceles(triangle_t t) {
    return is_valid(t) && (t.a == t.b || t.a == t.c || t.b == t.c);
}

bool is_scalene(triangle_t t) {
    return is_valid(t) && !is_isosceles(t);
}

bool is_valid(triangle_t t) {
    bool positive = t.a > 0.0 && t.b > 0.0 && t.c > 0.0;
    bool tri_eq_a = t.a <= t.b + t.c;
    bool tri_eq_b = t.b <= t.a + t.c;
    bool tri_eq_c = t.c <= t.a + t.b;
    return positive && tri_eq_a && tri_eq_b && tri_eq_c;
}

