impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        if matrix.len() == 0 || matrix[0].len() == 0 {
            return;
        }

        // we can use the first row and column as flags to indicate if the row or column should be set to zero`
        // which is a constant space solution

        // since we are using the leftmost column and top row as flags,
        // we run into the issue of the top-left corner needing to store two flags
        // which is not possible
        // so we use two booleans (constant space) for this 
        // and don't modify the top-left corner in the matrix
        let mut top_row = false;
        let mut top_col = false;
        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                if matrix[i][j] == 0 {
                    if j == 0 {
                        top_col = true;
                    } else {
                        matrix[0][j] = 0;
                    }
                    if i == 0 {
                        top_row = true;
                    } else {
                        matrix[i][0] = 0;
                    }
                } 
            }
        }

        for i in 1..matrix.len() {
            if matrix[i][0] == 0 {
                for j in 0..matrix[i].len() {
                    matrix[i][j] = 0;
                }
            }
        }
        for j in 1..matrix[0].len() {
            if matrix[0][j] == 0 {
                for i in 0..matrix.len() {
                    matrix[i][j] = 0;
                }
            }
        }
        if top_col {
            for i in 0..matrix.len() {
                matrix[i][0] = 0;
            }
        }
        if top_row {
            for j in 0..matrix[0].len() {
                matrix[0][j] = 0;
            }
        }
    }
}