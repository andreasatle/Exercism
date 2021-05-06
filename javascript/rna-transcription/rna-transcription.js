/** 
 * Given a DNA strand, return its RNA complement (per RNA transcription).
 * 
 * Both DNA and RNA strands are a sequence of nucleotides.
 * 
 * The four nucleotides found in DNA are adenine (A), cytosine (C), guanine (G) and thymine (T).
 * 
 * The four nucleotides found in RNA are adenine (A), cytosine (C), guanine (G) and uracil (U).
 * 
 * Given a DNA strand, its transcribed RNA strand is formed by replacing each nucleotide with its complement:
 * 
 * <p>G -> C
 * <p>C -> G
 * <p>T -> A
 * <p>A -> U
 * @file
 * @author Andreas Atle, atle.andreas@gmail.com
 */

// Mapping use for conversion from DNA to RNA.
const DNA_TO_RNA = {
  'G': 'C',
  'C': 'G',
  'T': 'A',
  'A': 'U'
}

/** 
 * toRna returns the RNA complement to a given DNA strand.
 * @func
 * @param {string} dna DNA strand.
 * @returns {string} Corresponding RNA strand.
 */
export const toRna = (dna) => {
  let rna = ""
  for (let i = 0; i < dna.length; i++) {
    rna += DNA_TO_RNA[dna[i]]
  }
  return rna
};

