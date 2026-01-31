impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        // for each N from 1..trunc(len / 2), check if each substring matches the pattern
        for p in 1..(1 + s.len() / 2) {
            // // this is a fun shortcut, but sadly exceeds time limit because of all the splitting
            // if s.split(&s[0..p + 1]).all(|s| s.is_empty()) {
            //     return true
            // }
            if s.len() % p != 0 {
                // p is not a divisor of s.len()
                continue;
            }

            let pattern = &s[0..p];
            for r in 0..(s.len() / p) {
                let slice =  &s[(r * p)..((r + 1) * p)];

                if pattern != slice {
                    break;
                }
                if r == s.len() / p - 1 {
                    return true;
                }
            }
        }
        false
    }
}