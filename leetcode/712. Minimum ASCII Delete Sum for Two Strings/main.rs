impl Solution {
    pub fn minimum_delete_sum(s1: String, s2: String) -> i32 {
        let mut dp = vec![vec![0; s2.len() + 1]; s1.len() + 1];
        let mut c1 = s1.as_bytes();
        let mut c2 = s2.as_bytes();

        for i in 0..=s1.len() {
            dp[i][s2.len()] = c1[i..s1.len()].iter().map(|&c| c as u32).sum();
        }

        for j in 0..=s2.len() {
            dp[s1.len()][j] = c2[j..s2.len()].iter().map(|&c| c as u32).sum();
        }

        for i in (0..s1.len()).rev() {
            for j in (0..s2.len()).rev() {
                if c1[i] == c2[j] {
                    dp[i][j] = dp[i + 1][j + 1];
                } else {
                    dp[i][j] =
                        std::cmp::min(c1[i] as u32 + dp[i + 1][j], c2[j] as u32 + dp[i][j + 1]);
                }
            }
        }

        dp[0][0] as i32
    }
}
