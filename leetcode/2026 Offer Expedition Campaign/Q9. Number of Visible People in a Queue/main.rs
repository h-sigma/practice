impl Solution {
    pub fn can_see_persons_count(heights: Vec<i32>) -> Vec<i32> {
        let mut sees : Vec<i32> = vec![0; heights.len()]; 

        // monotonic stack
        let mut stack = vec![];

        for i in (0..heights.len()).rev() {
            while !stack.is_empty() && *stack.last().unwrap() < heights[i] {
                // count all of the shorter neighbors we are popping off
                sees[i] += 1;
                stack.pop();
            }

            if !stack.is_empty() {
                // we can still see the taller person to our right we couldn't pop off
                sees[i] += 1;
            }
            
            stack.push(heights[i]);
        }

        sees
    }
}   