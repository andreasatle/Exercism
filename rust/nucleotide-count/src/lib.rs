//! Each of us inherits from our biological parents a set of chemical 
//! instructions known as DNA that influence how our bodies are 
//! constructed. All known life depends on DNA!
//! 
//! Note: You do not need to understand anything about nucleotides or 
//! DNA to complete this exercise.
//! 
//! DNA is a long chain of other chemicals and the most important are 
//! the four nucleotides, adenine, cytosine, guanine and thymine. A single 
//! DNA chain can contain billions of these four nucleotides and the order 
//! in which they occur is important! We call the order of these nucleotides 
//! in a bit of DNA a "DNA sequence".
//! 
//! We represent a DNA sequence as an ordered collection of these four 
//! nucleotides and a common way to do that is with a string of characters 
//! such as "ATTACG" for a DNA sequence of 6 nucleotides. 'A' for adenine, 
//! 'C' for cytosine, 'G' for guanine, and 'T' for thymine.
//! 
//! Given a string representing a DNA sequence, count how many of each 
//! nucleotide is present. If the string contains characters that aren't 
//! A, C, G, or T then it is invalid and you should signal an error.

use std::collections::HashMap;

/// Returns an result count of number of occurancies of the nucleotide in the dna-string.
pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {

    // Check that nucleotide is valid.
    if nucleotide != 'A' && nucleotide != 'C' && nucleotide != 'G' && nucleotide != 'T' {
        return Err(nucleotide);
    }

    let dna_count = nucleotide_counts(&dna);

    match dna_count {
        Ok(dna_count) => match dna_count.get(&nucleotide) {
            Some(cnt) => Ok(*cnt),
            None => Err(nucleotide),
        },
        Err(e) => Err(e),
    }
}

/// Returns result hash map with a count of each nucleotide for the dna string.
pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut dna_count = HashMap::new();
    dna_count.insert('A',0usize);
    dna_count.insert('C',0usize);
    dna_count.insert('G',0usize);
    dna_count.insert('T',0usize);
    for ch in dna.chars() {
            // Check that dna is valid.
            if ch != 'A' && ch != 'C' && ch != 'G' && ch != 'T' {
                return Err(ch);
            }
            let cnt = dna_count.entry(ch).or_insert(0);
        *cnt += 1;
    }
    Ok(dna_count)
}
