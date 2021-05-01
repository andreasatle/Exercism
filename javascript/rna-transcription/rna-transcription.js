// Mapping use for conversion from DNA to RNA.
const DNA_TO_RNA = {
  'G': 'C',
  'C': 'G',
  'T': 'A',
  'A': 'U'
}

// toRna returns the RNA complement to a given DNA strand.
export const toRna = (dna) => {
  let rna = ""
  for (let i = 0; i < dna.length; i++) {
    rna += DNA_TO_RNA[dna[i]]
  }
  return rna
};

