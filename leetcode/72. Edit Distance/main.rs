impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let word1 = word1.as_bytes();
        let word2 = word2.as_bytes();

        // dp[i][j] is the minimum number of operations to convert word1[0..i] to word2[0..j]
        // +1 to account for empty string
        let mut dp = vec![vec![0; word2.len() + 1]; word1.len() + 1];

        // when word2 is empty, we need to remove all characters from word1
        for i in 0..=word1.len() {
            dp[i][0] = i as i32;
        }

        // when word1 is empty, we need to add all characters from word2
        for j in 0..=word2.len() {
            dp[0][j] = j as i32;
        }

        for i in 1..=word1.len() {
            for j in 1..=word2.len() {
                if word1[i - 1] == word2[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1];
                } else {
                    // 1 + min(delete, insert, replace)
                    dp[i][j] = 1 + std::cmp::min(
                        std::cmp::min(
                            dp[i - 1][j], // delete
                            dp[i][j - 1], // insert
                        ),
                        dp[i - 1][j - 1], // replace
                    );
                }
            }
        }

        dp[word1.len()][word2.len()]
    }
}
