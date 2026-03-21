impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut min = 1;
        let mut max = 1;
        let mut res = nums[0];

        for &n in nums.iter() {
            if n == 0 {
                min = 1;
                max = 1;
                res = std::cmp::max(0, res);
                continue;
            }

            let tmp = max * n;

            max = std::cmp::max(
                n, 
                std::cmp::max(
                    n * max,
                    n * min,
                )
            );

            min = std::cmp::min(
                n,
                std::cmp::min(
                    tmp,
                    n * min,
                )
            );

            res = std::cmp::max(res, max);
        }

        res
    }
}       