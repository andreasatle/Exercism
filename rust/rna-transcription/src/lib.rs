//! Given a DNA strand, return its RNA complement (per RNA transcription).
//!
//! Both DNA and RNA strands are a sequence of nucleotides.
//!
//! The four nucleotides found in DNA are adenine (A), cytosine (C), guanine (G) and thymine (T).
//!
//! The four nucleotides found in RNA are adenine (A), cytosine (C), guanine (G) and uracil (U).
//!
//! Given a DNA strand, its transcribed RNA strand is formed by replacing each nucleotide with its complement:
//!
//! * G -> C
//! * C -> G
//! * T -> A
//! * A -> U

/// Contains the dna strand, consisting of 'A', 'C', 'G' and 'T'.
#[derive(Debug, PartialEq)]
pub struct Dna {
    dna: String,
}

/// Contains the rna strand, consisting of 'U', 'G', 'C' and 'A'.
#[derive(Debug, PartialEq)]
pub struct Rna {
    rna: String,
}

impl Dna {
    /// Creates a new result Dna.
    pub fn new(dna: &str) -> Result<Dna, usize> {
        for (i, ch) in dna.chars().enumerate() {
            if ch != 'A' && ch != 'C' && ch != 'G' && ch != 'T' {
                return Err(i);
            }
        }
        Ok(Dna {
            dna: dna.to_string(),
        })
    }

    /// Transcribes a Dna-strand to a Rna-strand.
    pub fn into_rna(self) -> Rna {
        let mut rna = String::new();
        for ch in self.dna.chars() {
            match ch {
                'A' => rna.push('U'),
                'C' => rna.push('G'),
                'G' => rna.push('C'),
                'T' => rna.push('A'),
                _ => (),
            }
        }
        return Rna { rna };
    }
}

impl Rna {
    // Creates a new result Rna.
    pub fn new(rna: &str) -> Result<Rna, usize> {
        for (i, ch) in rna.chars().enumerate() {
            if ch != 'U' && ch != 'G' && ch != 'C' && ch != 'A' {
                return Err(i);
            }
        }
        Ok(Rna {
            rna: rna.to_string(),
        })
    }
}
