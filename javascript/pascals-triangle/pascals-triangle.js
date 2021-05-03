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
