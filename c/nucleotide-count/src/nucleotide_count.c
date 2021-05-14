#include "nucleotide_count.h"

char *count(const char *dna_strand) {
    char *output = malloc(40);
    int a, c, g, t;
    a = c = g = t = 0;

    for (unsigned long i = 0; i < strlen(dna_strand); i++) {
        if (dna_strand[i] == 'A')
            a++;
        else if (dna_strand[i] == 'C')
            c++;
        else if (dna_strand[i] == 'G')
            g++;
        else if (dna_strand[i] == 'T')
            t++;
        else {
            output[0] = '\0';
            return output;
        }
    }
    sprintf(output, "A:%d C:%d G:%d T:%d", a, c, g, t);
    return output;
}
