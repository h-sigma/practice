impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut n = triangle.len();
        let mut sum = vec![0; triangle.len()];

        sum[0] = triangle[0][0];

        for row in 1..n {
            // why reverse? because we are accessing sum[col - 1]
            // if we go left to right, this will cause us to pick up the NEW sum calculated in the previous iteration of the array
            for col in (0..triangle[row].len()).rev() {
                sum[col] = match col {
                    // only valid parent is `col`
                    0 => sum[col] + triangle[row][col],
                    // valid parents are `col` and `col - 1`
                    // e.g., on (1, 1), only (0, 0) is a valid parent 
                    // but on (2, 1), (1, 1) is a valid parent
                    c if c < row => std::cmp::min(
                        sum[col - 1] + triangle[row][col],
                        sum[col] + triangle[row][col] 
                    ),
                    // only valid parent is `col - 1`
                    _ => sum[col - 1] + triangle[row][col]
                }
            }
        }

        sum.into_iter().min().unwrap()
    }
}