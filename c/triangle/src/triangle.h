// Determine if a triangle is equilateral, isosceles, or scalene.
// 
// An equilateral triangle has all three sides the same length.
// 
// An isosceles triangle has at least two sides the same length.
// (It is sometimes specified as having exactly two sides the same length,
// but for the purposes of this exercise we'll say at least two.)
// 
// A scalene triangle has all sides of different lengths.

#ifndef TRIANGLE_H
#define TRIANGLE_H

#include <stdbool.h>

typedef struct {
   double a;
   double b;
   double c;
} triangle_t;

// Check if triangle is equilateral
bool is_equilateral(triangle_t t);

// Check if triangle is isosceles (i.e. two sides the same)
bool is_isosceles(triangle_t t);

// Check if triangle is scalene (i.e. three different side-lengths)
bool is_scalene(triangle_t t);

// Check if triangle is valid (positive side lengths and obeys triangle inequality)
bool is_valid(triangle_t t);

#endif
