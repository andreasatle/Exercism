#include "rna_transcription.h"

char *to_rna(const char *dna) {
    int i, n;
    for (n = 0; dna[n] != '\0'; n++);
    char *rna = malloc(n*sizeof(dna));
    
    for (i = 0; i < n; i++) {
        if (dna[i] == 'G')
            rna[i] = 'C';
        else if (dna[i] == 'C')
            rna[i] = 'G';
        else if (dna[i] == 'T')
            rna[i] = 'A';
        else if (dna[i] == 'A')
            rna[i] = 'U';
    }
    rna[n] = '\0';
    return rna;
}
