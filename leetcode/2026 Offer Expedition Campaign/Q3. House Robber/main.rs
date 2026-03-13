impl Solution {
    /// DP by storing the maximum possible value if the current house is robbed and it's the last house (i.e., among left series only).
    /// We only need to check the i-2 and i-3 previous values, i.e. skip one house (standard condition) or skip two houses. If we check the i-4 value, it means we are skipping three houses. Of these three houses, we can rob i-2 if we rob i-4, satisfying the adjacency condition. Since house value is always positive, this will always lead to a more optimal solution.
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut max_if_robbed = nums.to_owned();
        let mut max_value = 0; 

        for i in 0..nums.len() {
            max_if_robbed[i] = std::cmp::max(
                max_if_robbed.get(i - 2).unwrap_or(&0),
                max_if_robbed.get(i - 3).unwrap_or(&0)
            ) + nums[i];
            max_value = std::cmp::max(max_value, max_if_robbed[i]);
        }

        max_value
    }
}