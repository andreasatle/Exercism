// Write a function that returns the earned points in a single toss of a Darts game.
// 
// Darts is a game where players throw darts to a target.
// 
// In our particular instance of the game, the target rewards with 4 different amounts
// of points, depending on where the dart lands:
// 
// If the dart lands outside the target, player earns no points (0 points).
// If the dart lands in the outer circle of the target, player earns 1 point.
// If the dart lands in the middle circle of the target, player earns 5 points.
// If the dart lands in the inner circle of the target, player earns 10 points.
// The outer circle has a radius of 10 units (This is equivalent to the total radius
// for the entire target), the middle circle a radius of 5 units, and the inner circle
// a radius of 1. Of course, they are all centered to the same point (That is, the
// circles are concentric) defined by the coordinates (0, 0).

#ifndef DARTS_H
#define DARTS_H

#include <stdint.h>

// Coordinate struct, required by test.
typedef struct {
    float x;
    float y;
} coordinate_t;

// Compute the dart-score depending on the coordinate.
uint8_t score(coordinate_t c);

#endif
