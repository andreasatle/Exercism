#include "resistor_color.h"

int color_code(resistor_band_t color) {
    return color;
}

resistor_band_t * colors(void) {
    static resistor_band_t A[10];
    A[0] = BLACK;
    A[1] = BROWN;
    A[2] = RED;
    A[3] = ORANGE;
    A[4] = YELLOW;
    A[5] = GREEN;
    A[6] = BLUE;
    A[7] = VIOLET;
    A[8] = GREY;
    A[9] = WHITE;
    return A;
}
