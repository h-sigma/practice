impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        // the minimum index before this day
        let mut min_upto = vec![0; prices.len()];
        // the index with maximum trade size (sell - buy)
        let mut max_idx = 0;

        for i in 1..(prices.len()) {
            if prices[i - 1] < prices[min_upto[i - 1]] {
                min_upto[i] = i - 1;
            } else {
                min_upto[i] = min_upto[i - 1];
            }

            if prices[max_idx] - prices[min_upto[max_idx]] < prices[i] - prices[min_upto[i]] {
                max_idx = i;
            } 
        }

        std::cmp::max(prices[max_idx] - prices[min_upto[max_idx]], 0)
    }
}   