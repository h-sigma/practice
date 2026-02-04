impl Solution {
    pub fn can_make_arithmetic_progression(arr: Vec<i32>) -> bool {
        let mut arr = arr.to_vec();
        arr.sort();
        let mut diff = None;
        // simply iterate the sorted array and check if the difference between consecutive elements is the same
        for i in 0..(arr.len() - 1) {
            match diff {
                None => {
                    diff = Some(arr[i + 1] - arr[i]);
                }
                Some(d) => {
                    diff = Some(arr[i + 1] - arr[i]);
                    if diff.unwrap() != d {
                        return false;
                    }
                }
            }
        }
        true
    }
}