impl Solution {
    pub fn largest_perimeter(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        // sort in decreasing order
        nums.sort_by(|a, b| b.cmp(a));
        
        // in each iteration we are going for a smaller triplet due to the sorting
        // so, the first triplet that is a valid triangle will be our answer
        for i in 0..(nums.len() - 2) {
            let a = nums[i];
            let b = nums[i+1];
            let c = nums[i+2];
            if a + b > c && a + c > b && b + c > a {
                return a + b + c;
            }
            // explanation:
            // given any index triplet (i, j, k), due to the sorted nature of the nums array,
            // sum(i, j, k) > sum(i + 1, j, k) ; sum(i, j, k) > sum(i, j + 1, k); sum(i, j, k) > sum(i, j, k + 1)
            // so, first of all: the largest triplet is one with consecutive indices, i.e. j = i + 1, k = i + 2
            // however, such triplets may be invalid. so why do we not try j > (i + 1) or k > (i + 2) in order to find a valid triplet?
            // due to sorting, nums[j + 1] < nums[j] and nums[k + 1] < nums[k]
            // if (b + c > a) is false, i.e. (b + c <= a), a decreasing b will not change the result of the condition
            // if (a + b > c) is false, i.e. (a + b <= c), a decreasing b will not change the result of the condition
            // the last condition, (a + c > b) cannot be false because we know that c >= b and b >= a
            // thus, there cannot be a valid triplet at j > (i + 1)
            // in a similar way, we can explain why there will be no valid tripl
        }
        0
    }
}