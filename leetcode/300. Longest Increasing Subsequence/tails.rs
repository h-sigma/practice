impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut tails = vec![];

        // build tails array
        // this array is sorted in ascending order
        // each elements tails[i] is the smallest number in nums that is greater than or equal to tails[i-1]
        // meaning it's the smallest number that can form a tail of length i+1

        for i in 0..n {
            let partition = tails.partition_point(|v| *v < nums[i]);
            if partition == tails.len() {
                tails.push(nums[i]);
            } else {
                tails[partition] = nums[i];
            }
        }

        tails.len() as i32
    }
}
