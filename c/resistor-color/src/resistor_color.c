#include "resistor_color.h"

int color_code(resistor_band_t color) {
    return color;
}

resistor_band_t * colors(void) {
    static resistor_band_t A[10] = {BLACK, BROWN, RED, ORANGE, YELLOW, GREEN, BLUE, VIOLET, GREY, WHITE};
    return A;
}
