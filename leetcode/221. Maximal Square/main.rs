impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let rows = matrix.len();
        let cols = matrix[0].len();

        let mut size = vec![0; cols + 1];

        let mut max_side = *size.iter().max().unwrap();
        let mut prev_diag = 0;

        for r in 0..rows {
            for c in 1..(cols + 1) {
                let temp = size[c];

                // recurrence relation: min of top, left, and diagonal
                // to make a k side square whose bottom right corner is at r,c
                // we need to have k-1 side squares at r-1,c r,c-1 r-1,c-1 cells
                if matrix[r][c - 1] == '1' {
                    size[c] = std::cmp::min(std::cmp::min(size[c], size[c - 1]), prev_diag);
                    size[c] += 1;
                    max_side = std::cmp::max(max_side, size[c]);
                } else {
                    size[c] = 0;
                }

                prev_diag = temp;
            }
            prev_diag = 0; // reset for next row
        }

        max_side * max_side
    }
}
