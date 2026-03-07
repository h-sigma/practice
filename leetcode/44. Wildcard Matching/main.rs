impl Solution {
    // this is a greedy solution that uses a few pointers to keep track of the current position in the string and pattern
    // most efficient solution, 0ms runtime, 2MB memory usage

    // the key is to realize that a '*' can match at most until the end of the sequence or the next '*' (the next '*' recurses on this condition to handle matching more characters until the end of the sequence, or...)
    pub fn is_match(s: String, p: String) -> bool {
        let s = s.chars().collect::<Vec<_>>();
        let p = p.chars().collect::<Vec<_>>();

        let mut sidx = 0;
        let mut pidx = 0;

        let mut last_asterik: Option<usize> = None;
        let mut last_sidx = 0;

        while sidx < s.len() {
            // matching character and '?' both advance positions
            if pidx < p.len() && (p[pidx] == s[sidx] || p[pidx] == '?') {
                sidx += 1;
                pidx += 1;
            }
            // save '*' position, match it to 0 characters for now 
            // it doesn't matter if we already have a '*' behind us
            // because the pattern between the two '*' is already matched
            // and the previous '*' can't "do any better" than the current '*' for
            // the upcoming string
            else if pidx < p.len() && p[pidx] == '*' {
                last_sidx = sidx;
                last_asterik = Some(pidx);
                pidx += 1;
            } 
            // no match, but we have a '*' to fall back to and match "more" characters
            else if let Some(last_asterik) = last_asterik {
                last_sidx += 1;
                sidx = last_sidx;
                pidx = last_asterik + 1;
            } 
            // dead end; no match and no wildcard to fall back on
            else {
                return false;
            }
        }

        while pidx < p.len() && p[pidx] == '*' {
            pidx += 1;
        }

        pidx == p.len()
    }
}