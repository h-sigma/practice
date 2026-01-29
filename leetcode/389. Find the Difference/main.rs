use std::collections::HashMap;

impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut chars = HashMap::new();
        for ch in s.chars() {
            chars.entry(ch).and_modify(|counter| *counter += 1).or_insert(1);
        }
        for ch in t.chars() {
            chars.entry(ch).and_modify(|counter| *counter -= 1).or_insert(-1);
        }

        for (ch, count) in &chars {
            if *count < 0 {
                return *ch;
            }
        }

        panic!("No letter was added. Incorrect test case.");
    }
}