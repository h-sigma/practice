impl Solution {
    /// Uses memoization and a recursive approach to check if the string can be segmented into a space-separated sequence of one or more dictionary words.
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut memo = vec![None; s.len()];
        Solution::find_match(&s[..], 0, &word_dict, &mut memo)
    }

    fn find_match(s: &str, idx: usize, word_dict: &[String], memo: &mut Vec<Option<bool>>) -> bool {
        if let Some(result) = memo[idx] {
            return result;
        }
        for (word_idx, word) in word_dict.iter().enumerate() {
            if s[idx..].starts_with(word) {
                let new_start = idx + word.len();
                if new_start >= s.len() {
                    memo[idx] = Some(true);
                    return true;
                }
                let result = Solution::find_match(s, new_start, word_dict, memo);
                if result {
                    memo[idx] = Some(true);
                    return true;
                }
            }
        }
        memo[idx] = Some(false);
        return false;
    }
}
