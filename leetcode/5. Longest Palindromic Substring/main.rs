impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let chars = s.chars().collect::<Vec<char>>();
        // memoize whether smaller substrings are palindrome or not
        let mut is_palindrome = vec![vec![false; chars.len()]; chars.len()];

        for i in 0..chars.len() {
            is_palindrome[i][i] = true;
        }

        let mut max_len = 0;
        let mut max_i = 0;
        let mut max_j = 0;
        // for "abac", iterate over substrings in order "c", "a", "ac", "b", "ba", "bac", "a", "ab", "aba", "abac"
        // this is in the right order for each substring's inner substring to have already been checked
        for i in (0..chars.len()).rev() {
            for j in i..chars.len() {
                // if ends of palindrome are the same character and
                // either the substring is too short or inner substring is a palindrome
                if chars[i] == chars[j] && ((j - i < 2) || is_palindrome[i + 1][j - 1]) {
                    is_palindrome[i][j] = true;

                    // keep a running of largest palindrome found
                    let len = j - i + 1;
                    if len > max_len {
                        max_len = len;
                        max_i = i;
                        max_j = j;
                    }
                }
            }
        }

        chars[max_i..=max_j].iter().collect::<String>()
    }
}
