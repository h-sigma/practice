impl Solution {
    pub fn count_odds(low: i32, high: i32) -> i32 {
        match (low % 2, high % 2) {
            (1, 0) | (0, 1) => (high - low + 1) / 2,
            (0, 0) => (high - low) / 2,
            (1, 1) => 1 + (high - low) / 2,
            _ => panic!("Woah.")
        }
    }
}