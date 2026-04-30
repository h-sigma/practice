impl Solution {
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        let max_value = nums.iter().map(|i| *i).max().unwrap_or(0);
        let mut buckets = vec![0; max_value as usize + 1];

        for i in 0..nums.len() {
            buckets[nums[i] as usize] += nums[i];
        }

        for i in 2..buckets.len() {
            buckets[i] = std::cmp::max(
                buckets[i - 2] + buckets[i],
                buckets[i - 1],
            );
        }

        buckets.iter().map(|value| *value).max().unwrap_or(0)
    }
} 