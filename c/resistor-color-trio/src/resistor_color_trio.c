#include "resistor_color_trio.h"

// Returns the value of the resistor.
resistor_value_t color_code(resistor_band_t *band) {
    resistor_value_t res;

    // Compute the value from the first two bands.
    res.value = band[0]*10 + band[1];

    // Add zeros to the value.
    for (int i = band[2]; i>0; i--) {
        res.value *= 10;
    }

    // Correct the value for unit.
    for (res.unit = 0; res.value >= 1000; res.unit++) {
        res.value /= 1000;
    }

    return res;
}
