/**
 * Given a string representing a matrix of numbers, return the rows and columns
 * of that matrix.
 * @file
 * @author Andreas Atle, atle.andreas@gmail.com
 */

/**
 * Class representation of matrix.
 * @class 
 */
export class Matrix {
  matrix;
  transpose;
  
  /** 
   * @constructor
   * @param {string} matrix String representation of a matrix.
   */
  constructor(matrix) {
    this.matrix = [];
    // Alternative way of writing a for-loop in js.
    matrix.split('\n').forEach((line, i) => {
      let row = [];
      line.split(' ').forEach((val, j) => {
        row.push(Number(val));
      })
      this.matrix.push(row);
    })

    // Compute the transpose of the matrix.
    this.transpose = [];

    for (let j = 0; j < this.matrix[0].length; j++) {
      this.transpose[j] = [];
    }

    for (let row of this.matrix) {
      for (let j = 0; j < row.length; j++) {
        this.transpose[j].push(row[j]);
      }
    }
  }

  /**
   * Get the rows of the matrix.
   * @public
   * @method
   * @returns {number[][]}
   */
  get rows() {
    return Object.assign({}, this.matrix);
  }

  /**
   * Get the columns of the matrix.
   * @public
   * @method
   * @returns {number[][]}
   */
   get columns() {
    return Object.assign({}, this.transpose)
  }
}
