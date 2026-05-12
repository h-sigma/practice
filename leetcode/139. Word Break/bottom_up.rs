impl Solution {
    /// Uses a bottom-up approach.
    /// `can_be_segmented[i]` is true if the substring `s[0..i]` can be segmented into dictionary words
    /// `can_be_segmented[0]` is true as trivial case because it is the empty string
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut can_be_segmented = vec![false; s.len() + 1];
        can_be_segmented[0] = true;

        for i in 0..s.len() {
            if !can_be_segmented[i] {
                continue;
            }
            for word in word_dict.iter() {
                if (i + word.len() < s.len() + 1) && can_be_segmented[i + word.len()] {
                    // if we already know this substring can be segmented, we can skip the current word
                    // this saves us from checking the same substring multiple times
                    continue;
                }
                if s[i..].starts_with(word) {
                    can_be_segmented[i + word.len()] = true;
                }
            }
        }

        can_be_segmented[s.len()]
    }
}
