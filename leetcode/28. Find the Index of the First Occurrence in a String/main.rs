impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        // I mean, we could use an in-built function, but I won't ^^'
        // Let's go with a naive implementation O(N * k)
        // And compare as bytes to avoid calling char functions all the time
        let nb = needle.as_bytes();
        let hb = haystack.as_bytes();
        if nb.len() > hb.len() {
            return -1;
        }
        for outer_idx in 0..(hb.len() - nb.len() + 1) {
            for inner_idx in 0..(nb.len()) {
                if hb[outer_idx + inner_idx] != nb[inner_idx] {
                    break;
                }
                if inner_idx == needle.len() - 1 {
                    return outer_idx.try_into().unwrap();
                }
            }
        }
        return -1;
    }
}