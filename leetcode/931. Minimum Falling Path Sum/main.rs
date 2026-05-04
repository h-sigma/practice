impl Solution {
    pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
        let n = matrix.len();
        let mut sum = matrix[0].to_owned();

        let mut prev = 0;

        for row in 1..n {
            // start of iteration - reset prev to MAX
            prev = i32::MAX;
            for col in 0..n {
                // save the value because it is going to be overwritten soon
                let curr = sum[col];

                sum[col] = matrix[row][col];

                sum[col] += std::cmp::min(
                    std::cmp::min(prev, curr),
                    *sum.get(col + 1).unwrap_or(&i32::MAX),
                );

                prev = curr;
            }
        }

        sum.into_iter().min().unwrap()
    }
}
