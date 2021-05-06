/**
 * Determine if a sentence is a pangram. A pangram 
 * (Greek: παν γράμμα, pan gramma, "every letter")
 * is a sentence using every letter of the alphabet at least once.
 * The best known English pangram is:
 * 
 * The quick brown fox jumps over the lazy dog.
 * 
 * The alphabet used consists of ASCII letters a to z, inclusive, 
 * and is case insensitive. Input will not contain non-ASCII symbols.
 * @file
 * @author Andreas Atle, atle.andreas@gmail.com
 */

/**
 * Determine whether the sentence is a pangram, i.e. contains all letters
 * in the english alphabet.
 * @func
 * @param {string} sentence Sentence to be tested for pangram.
 * @returns {bool} true if the sentence is a pangram.
 */
export const isPangram = (sentence) => {
  let charSet = new Set();
  for (let ch of 'abcdefghijklmnopqrstuvwxyz') {
    charSet.add(ch);
  }
  for (let ch of sentence.toLowerCase()) {
    charSet.delete(ch);
  }
  return charSet.size === 0;
};
