#include "nucleotide_count.h"

enum indices {INDEX_A, INDEX_C, INDEX_G, INDEX_T};
char *count(const char *dna_strand) {
    char *output = malloc(40);
    int counter[4] = {0, 0, 0, 0};

    for (unsigned long i = 0; i < strlen(dna_strand); i++) {
        if (dna_strand[i] == 'A')
            counter[INDEX_A]++;
        else if (dna_strand[i] == 'C')
            counter[INDEX_C]++;
        else if (dna_strand[i] == 'G')
            counter[INDEX_G]++;
        else if (dna_strand[i] == 'T')
            counter[INDEX_T]++;
        else {
            output[0] = '\0';
            return output;
        }
    }
    sprintf(output, "A:%d C:%d G:%d T:%d", 
        counter[INDEX_A],
        counter[INDEX_C],
        counter[INDEX_G],
        counter[INDEX_T]);
    return output;
}
