// Each of us inherits from our biological parents a set of chemical instructions
// known as DNA that influence how our bodies are constructed. All known life
// depends on DNA!
// 
// DNA is a long chain of other chemicals and the most important are the four
// nucleotides, adenine, cytosine, guanine and thymine. A single DNA chain can
// contain billions of these four nucleotides and the order in which they occur
// is important! We call the order of these nucleotides in a bit of DNA a
// "DNA sequence".
// 
// We represent a DNA sequence as an ordered collection of these four nucleotides
// and a common way to do that is with a string of characters such as "ATTACG"
// for a DNA sequence of 6 nucleotides. 'A' for adenine, 'C' for cytosine,
// 'G' for guanine, and 'T' for thymine.
// 
// Given a string representing a DNA sequence, count how many of each nucleotide
// is present. If the string contains characters that aren't A, C, G, or T then
// it is invalid and you should signal an error.

#ifndef _NUCLEOTIDE_COUNT_H
#define _NUCLEOTIDE_COUNT_H

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

char *count(const char *dna_strand);

#endif
