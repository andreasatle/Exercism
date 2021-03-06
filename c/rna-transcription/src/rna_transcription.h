// Given a DNA strand, return its RNA complement (per RNA transcription).
// 
// Both DNA and RNA strands are a sequence of nucleotides.
// 
// The four nucleotides found in DNA are:
// adenine (A), cytosine (C), guanine (G) and thymine (T).
// 
// The four nucleotides found in RNA are:
//  adenine (A), cytosine (C), guanine (G) and uracil (U).
// 
// Given a DNA strand, its transcribed RNA strand is formed by replacing
// each nucleotide with its complement:
// 
// G -> C
// C -> G
// T -> A
// A -> U

#ifndef RNA_TRANSCRIPTION_H
#define RNA_TRANSCRIPTION_H

#include <stdlib.h>
#include <string.h>

// Find the rna strand for the given dna strand.
char *to_rna(const char *dna);

#endif
