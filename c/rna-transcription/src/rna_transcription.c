#include "rna_transcription.h"

char *to_rna(const char *dna) {
    unsigned long i;
    char *rna = malloc(strlen(dna)+1);
    
    for (i = 0; i < strlen(dna); i++) {
        if (dna[i] == 'G')
            rna[i] = 'C';
        else if (dna[i] == 'C')
            rna[i] = 'G';
        else if (dna[i] == 'T')
            rna[i] = 'A';
        else if (dna[i] == 'A')
            rna[i] = 'U';
    }
    rna[i] = '\0';
    return rna;
}
