impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp = vec![1; n];

        let mut max_dp = 1;
        for i in (0..n).rev() {
            let mut max_j = 0;
            for j in (i + 1)..n {
                if nums[j] > nums[i] {
                    max_j = std::cmp::max(max_j, dp[j]);
                }
            }
            dp[i] = 1 + max_j;
            max_dp = std::cmp::max(max_dp, dp[i]);
        }

        max_dp
    }
}
