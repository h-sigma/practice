impl Solution {
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let n = s.len();
        let chars: Vec<char> = s.chars().collect();
        let mut dp = vec![0; n];

        for r in (0..n).rev() {
            dp[r] = 1;
            let mut prev_diagonal = 0;

            for c in (r + 1)..n {
                let temp = dp[c];
                if chars[r] == chars[c] {
                    dp[c] = 2 + prev_diagonal;
                } else {
                    dp[c] = std::cmp::max(dp[c], dp[c - 1]);
                }
                prev_diagonal = temp;
            }
        }

        dp[n - 1]
    }
}