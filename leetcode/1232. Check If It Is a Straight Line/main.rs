impl Solution {
    pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
        if coordinates.len() < 2 {
            return true;
        }
        let first_slope = Solution::slope(&coordinates[1], &coordinates[0]);
        const EPSILON: f64 = 0.01;
        for i in 2..coordinates.len() {
            let curr_slope = Solution::slope(&coordinates[i - 1], &coordinates[i]);
            if (first_slope - curr_slope).abs() > EPSILON {
                return false;
            }
        }
        true
    }

    fn slope(from: &Vec<i32>, to: &Vec<i32>) -> f64 {
        if to[0] == from[0] {
            return f64::INFINITY;
        }
        (to[1] as f64 - from[1] as f64) / (to[0] as f64 - from[0] as f64)
    }
}