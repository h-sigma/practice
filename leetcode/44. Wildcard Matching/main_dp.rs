// this is a dynammic programming solution that uses a set to keep track of failed substring pairs
// it's not the most efficient solution but it passes all test cases in 100ms (which is quite slow)
use std::collections::HashSet;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        //  convert to vec of chars for easy access
        let s: Vec<char> = s.chars().collect();
        let mut p: Vec<char> = p.chars().collect();

        // collapse consecutive *** into one *
        p.dedup_by(|a, b| *a == '*' && *b == '*');

        // in this set, we will keep track of failed substring pairs
        // to avoid re-computing dead recursion subtrees
        let mut failed: HashSet<(usize, usize)> = HashSet::new();

        Solution::match_partial(&s[..], &p[..], &mut failed)
    }

    fn match_partial(s: &[char], p: &[char], failed: &mut HashSet<(usize, usize)>) -> bool {
        if failed.contains(&(s.len(), p.len())) {
            return false;
        }

        //  iterate over string and pattern
        let mut sidx = 0;
        for pidx in 0..p.len() {

            //println!("{:?} __ {:?}", &s[std::cmp::min(sidx, s.len())..], &p[std::cmp::min(pidx, p.len())..]);

            match p[pidx] {
                x if sidx < s.len() && x == s[sidx] => {
                    sidx += 1;
                },
                '?' if sidx < s.len() => {
                    sidx += 1;
                },
                '*' => {
                    if pidx == p.len() - 1 {
                        // * as last character will match anything
                        return true;
                    }

                    for subidx in sidx..=s.len() {
                        if Solution::match_partial(&s[subidx..s.len()], &p[(pidx+1)..], failed) {
                            return true;
                        }
                        failed.insert((
                            s.len() - subidx,
                            p.len() - (pidx + 1)
                        ));
                    }
                    return false;
                },
                _ => {
                    return false;
                }
            }
        }

        sidx == s.len()
    }
}