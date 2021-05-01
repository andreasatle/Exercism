const chars = 'abcdefghijklmnopqrstuvwxyz';
export const isPangram = (sentence) => {
  let charSet = new Set();
  for (let ch of chars) {
    charSet.add(ch);
  }
  for (let ch of sentence.toLowerCase()) {
    charSet.delete(ch);
  }
  return charSet.size === 0;
};
