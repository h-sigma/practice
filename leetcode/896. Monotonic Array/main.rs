use std::cmp::Ordering;

impl Solution {
    pub fn is_monotonic(nums: Vec<i32>) -> bool {
        if nums.len() == 1 {
            return true;
        }
        // save current order of array
        let mut ord = None;
        for i in 0..(nums.len() - 1) {
            let cur = nums[i].cmp(&nums[i+1]);
            match (ord, cur) {
                // If the order reverses, it is not monotonically inc/dec
                (Some(Ordering::Less), Ordering::Greater) | (Some(Ordering::Greater), Ordering::Less) => return false,
                // Only update the order once when we know it (and it's not Ordering::Equal)
                (None, Ordering::Greater) | (None, Ordering::Less) => {
                    ord = Some(cur);
                }
                _ => {}
            }
        }
        return true;
    }
}