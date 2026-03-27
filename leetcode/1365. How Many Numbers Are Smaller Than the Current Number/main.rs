impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        // use counting sort
        let mut lt = vec![0; 101]; // 0 <= nums[i] <= 100

        for num in &nums {
            // found X, which means there is one number smaller than X+1
            if let Some(n) = lt.get_mut((num + 1) as usize) {
                *n = *n + 1;
            }
        }

        // aggregate the count of smaller numbers
        for i in 1..lt.len() {
            lt[i] += lt[i - 1];
        }

        nums.iter().map(|num| lt[num.clone() as usize].clone()).collect()
    }
}