impl Solution {
    //  using greedy solution
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut max_reachable = 0;

        for i in 0..nums.len() {
            if i > max_reachable {
                // cannot reach the current index
                return false;
            }

            max_reachable = std::cmp::max(max_reachable, i + nums[i] as usize);
        }

        max_reachable >= nums.len() - 1
    }

    // using bottom-up DP
    // pub fn can_jump(nums: Vec<i32>) -> bool {
    //     let mut cache: Vec<Option<bool>> = vec![None; nums.len()];

    //     Solution::can_reach(&nums, 0, &mut cache)
    // }

    // fn can_reach(nums: &[i32], index: usize, cache: &mut[Option<bool>]) -> bool {
    //     if let Some(reaches) = cache[index] {
    //         return reaches;
    //     }

    //     if index == nums.len() - 1 {
    //         // end of array is reachable from the end, duh.
    //         return true;
    //     }

    //     for i in 1..=nums[index] {
    //         if(Solution::can_reach(nums, index + i as usize, cache)) {
    //             cache[index] = Some(true);
    //             return true;
    //         }
    //     }

    //     cache[index] = Some(false);
    //     return false;
    // }

    // using top-down DP
    // pub fn can_jump(nums: Vec<i32>) -> bool {
    //     let mut reachable = vec![false; nums.len()];

    //     Solution::mark_reachable(&nums, 0, &mut reachable);

    //     reachable[reachable.len() - 1]
    // }

    // fn mark_reachable(nums: &[i32], index: usize, reachable: &mut[bool]) {
    //     if index >= nums.len() || reachable[index] {
    //         return;
    //     }

    //     reachable[index] = true;

    //     for i in 1..=nums[index] {
    //         Solution::mark_reachable(nums, index + i as usize, reachable);
    //     }
    // }
}