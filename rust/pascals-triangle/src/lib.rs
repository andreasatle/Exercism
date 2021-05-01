// PascalsTriangle contains the elements of Pascals triangle.
pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>
}

impl PascalsTriangle {
    // new creates a new instance of the struct PascalsTriangle.
    pub fn new(row_count: u32) -> Self {
        let mut pascal = PascalsTriangle{rows: Vec::new()};
        
        if row_count >= 1 {
            pascal.rows.push(vec![1])
        }
        if row_count >= 2 {
            pascal.rows.push(vec![1,1])
        }
        for i in 2..row_count as usize {
            // Add the first 1 in row i
            pascal.rows.push(vec![1]);

            // Add the interior points in row i using row i-1.
            for j in 1..pascal.rows[i-1].len() {
                let left = pascal.rows[i-1][j-1];
                let right = pascal.rows[i-1][j];
                pascal.rows[i].push(left+right);
            }
            // Add the last 1 in row i
            pascal.rows[i].push(1);
        }
        pascal
    }

    // rows returns a copy of the rows of Pascals triangle.
    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut out: Vec<Vec<u32>> = Vec::new();
        for row in &self.rows {
            out.push(row.clone());
        }
        out
    }
}
