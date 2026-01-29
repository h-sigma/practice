impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut result = String::with_capacity(word1.len() + word2.len());
        let max_len = std::cmp::max(word1.len(), word2.len());
        let mut word1_chars = word1.chars();
        let mut word2_chars = word2.chars();
        for i in 0..max_len {
            if i < word1.len() {
                result.push(word1_chars.next().unwrap());
            }
            if i < word2.len() {
                result.push(word2_chars.next().unwrap());
            }
        }
        result
    }
}