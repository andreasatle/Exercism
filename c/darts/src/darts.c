#include "darts.h"

// Compute the dart-score depending on the coordinate.
uint8_t score(coordinate_t c) {

    float square_distance = c.x*c.x + c.y*c.y;

    // Inside inner circle.
    if (square_distance <= 1.0) {
        return 10;
    }

    // Inside middle circle.
    if (square_distance <= 25.0) {
        return 5;
    }

    // Inside outer circle.
    if (square_distance <= 100.0) {
        return 1;
    } 

    // Outside outer circle.
    return 0;
}
