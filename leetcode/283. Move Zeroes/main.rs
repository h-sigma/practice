impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut left = 0;

        for right in 0..nums.len() {
            // swap all non-zero elements to the left
            if nums[right] != 0 {
                // swap
                nums.swap(left, right);
                left += 1;
            }
        }
    }
}