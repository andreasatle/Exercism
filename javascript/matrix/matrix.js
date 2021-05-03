//
// This is only a SKELETON file for the 'Matrix' exercise. It's been provided as a
// convenience to get you started writing code faster.
//

export class Matrix {
  matrix;
  transpose;
  constructor(matrix) {
    this.matrix = [];
    matrix.split('\n').forEach((line, i) => {
      let row = [];
      line.split(' ').forEach((val, j) => {
        row.push(Number(val));
      })
      this.matrix.push(row);
    })
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

  get rows() {
    return Object.assign({}, this.matrix);
  }

  get columns() {
    return Object.assign({}, this.transpose)
  }
}
