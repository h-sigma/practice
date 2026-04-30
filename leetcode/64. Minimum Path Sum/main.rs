impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let cols = grid[0].len();
        let rows = grid.len();
        let mut sum = vec![0; cols];

        // top row - can only go right
        for i in 0..cols {
            sum[i] = grid[0][i] + sum.get(i - 1).unwrap_or(&0);
        }

        for i in 1..rows {
            // left column - can only go down
            sum[0] = sum[0] + grid[i][0];
            for j in 1..cols {
                sum[j] = std::cmp::min(sum[j] + grid[i][j], sum[j - 1] + grid[i][j]);
            }
        }

        sum[cols - 1]
    }
}
