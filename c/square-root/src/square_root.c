#include "square_root.h"

int square_root(int n) {
    int x = n, x2 = x+1, x3 = x+2;
    for (;x!=x2 && x!=x3;) {
        x3 = x2;
        x2 = x;
        x = (x+n/x)/2;
    }
    return x;
}
