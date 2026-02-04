impl Solution {
    pub fn array_sign(nums: Vec<i32>) -> i32 {
        // need to call sign_func on each component of the array
        // because otherwise we can overflow i32 (or even i64)
        // since the product can be 100^1000 for given constraints
        (nums.iter().fold(1, |acc, &b| acc * Solution::sign_func(b)))
    }

    fn sign_func(x: i32) -> i32 {
        match x.cmp(&0) {
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => 1,
        }
    }
}