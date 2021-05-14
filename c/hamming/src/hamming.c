#include "hamming.h"

int compute(const char *lhs, const char *rhs){
    int cnt = 0;
    int i = 0;
    for (;;i++) {
        if (lhs[i] == '\0' || rhs[i] == '\0') {
            break;
        }
        if (lhs[i] != rhs[i]) {
            cnt++;
        }
    }
    return lhs[i] == '\0' && rhs[i] == '\0' ? cnt : -1;
}

