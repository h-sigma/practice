impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        // Using a map, count how many times each letter occurs in both strings.

        let mut map = std::collections::HashMap::new();

        for ch in s.chars() {
            map.entry(ch).and_modify(|count| *count += 1).or_insert(1);
        }

        for ch in t.chars() {
            map.entry(ch).and_modify(|count| *count -= 1).or_insert(-1);
        }

        map.into_values().all(|count| count == 0)
    }
}