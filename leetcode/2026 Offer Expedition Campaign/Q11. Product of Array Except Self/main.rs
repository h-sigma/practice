impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut product = vec![1; nums.len()];

        let mut running = 1;
        for i in 0..nums.len() {
            product[i] = running;
            running *= nums[i];
        }

        running = 1;
        for i in (0..nums.len()).rev() {
            product[i] *= running;
            running *= nums[i];
        }

        product
    }
}