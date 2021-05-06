/**
 * Compute Pascal's triangle up to a given number of rows.
 * 
 * In Pascal's Triangle each number is computed by adding the numbers to the right and left of the current position in the previous row.
 * 
 * <p>     1
 * <p>    1 1
 * <p>   1 2 1
 * <p>  1 3 3 1
 * <p> 1 4 6 4 1
 * @file
 * @author Andreas Atle, atle.andreas@gmail.com
 */

/**
 * Compute the first n rows in Pascals triangle.
 * @func
 * @param {number} n Number of rows.
 * @returns {number[][]} The first n rows of Pascals triangle.
 */
export const rows = (n) => {
  let pascal = [];
  if (n == 0) {
    return pascal;
  }
  pascal.push([1]);
  if (n == 1) {
    return pascal;
  }
  pascal.push([1, 1]);
  if (n == 2) {
    return pascal;
  }

  for (let i = 2; i < n; i++) {
    pascal.push([1]);
    for (let j = 1; j < i; j++) {
      pascal[i].push(pascal[i-1][j-1]+pascal[i-1][j]);
    }
    pascal[i].push(1);
  }
  return pascal
};
