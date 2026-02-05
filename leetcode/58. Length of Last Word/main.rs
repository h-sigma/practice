impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        // iterators in rust are such a pleasure to compose.
        s
            .split(' ')
            .rev()
            .filter(|&s| s.len() > 0)
            .next()
            .map_or(0, |s| s.len() as i32)
    }
}