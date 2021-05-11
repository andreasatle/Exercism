'''Given a DNA strand, return its RNA complement (per RNA transcription).

Both DNA and RNA strands are a sequence of nucleotides.

The four nucleotides found in DNA are adenine (A), cytosine (C), guanine (G) and thymine (T).

The four nucleotides found in RNA are adenine (A), cytosine (C), guanine (G) and uracil (U).

Given a DNA strand, its transcribed RNA strand is formed by replacing each nucleotide with its complement:

G -> C
C -> G
T -> A
A -> U
'''

_DNA_TO_RNA = {'G': 'C', 'C': 'G', 'T': 'A', 'A': 'U'}


def to_rna(dna_strand):
    '''Convert DNA-strand to RNA-strand.'''
    rna_strand = ""
    for ch in dna_strand:
        if ch in _DNA_TO_RNA:
            rna_strand += _DNA_TO_RNA[ch]
        else:
            raise ValueError(ch)
    return rna_strand
