impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let mut dp = vec![vec![0; t.len() + 1]; s.len() + 1];
        let sb = s.as_bytes();
        let tb = t.as_bytes();

        let m = s.len();
        let n = t.len();

        // base cases
        for i in 0..=m {
            dp[i][0] = 1;
        }
        // everything already initialized to 0, so we skip this
        //for j in 1..=n {
        //    dp[0][j] = 0;
        //}

        for i in 1..=m {
            for j in 1..=n {
                if sb[i - 1] == tb[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + dp[i - 1][j];
                } else {
                    dp[i][j] = dp[i - 1][j];
                }
            }
        }

        dp[m][n]
    }
}
