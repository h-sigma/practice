impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut write_index = 1;
        for i in 1..nums.len() {
            if nums[i] != nums[i - 1] {
                nums[write_index] = nums[i]; // don't swap, we don't care about duplicates
                write_index += 1;
            }
        }
        write_index as i32
    }
}