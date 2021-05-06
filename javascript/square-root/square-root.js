/**
 * Given a natural radicand, return its square root.
 * 
 * Note that the term "radicand" refers to the number for which the root is to be determined. That is, it is the number under the root symbol.
 * 
 * Check out the Wikipedia pages on square root and methods of computing square roots.
 * 
 * Recall also that natural numbers are positive real whole numbers (i.e. 1, 2, 3 and up).
 * @file
 * @author Andreas Atle, atle.andreas@gmail.com
 */

/**
 * Computes the integer square root of a number.
 * @func
 * @param {int} x Argument to square root. 
 * @returns integer square root of the argument.
 * @throws {Error} squareRoot argument is not a number
 * @throws {Error} squareRoot of negative number
 */
export const squareRoot = (x) => {
  if (typeof x !== 'number') {
    throw new Error('squareRoot argument is not a number');
  }
  if (x < 0) {
    throw new Error('squareRoot of negative number');
  }

  // Use integer version of Newton's method.
  // There are cases where the integer squareroot ends in the "2-loop".
  // This is detected by storing the two latest values.
  // Its a bit devious, it passed all the tests without this...
  let y = 1;
  let yOld = 2;
  let yOld2 = 3;
  while (y != yOld && y != yOld2) {
    yOld2 = yOld
    yOld = y;
    y = Math.floor((y+x/y)/2);
  }
  return y;
};
