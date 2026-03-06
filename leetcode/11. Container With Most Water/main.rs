impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut short = 0;
        let mut long = height.len() - 1;
        let mut max_area = i32::MIN;

        while (short < long) {
            let area = Solution::area(&height, short, long);
            if area > max_area {
                max_area = area;
            }
            // always move the the pointer that points to the shorter line
            // because the area is limited by the shorter line
            // we *may* discover a larger area if we move the short pointer,
            // but we *cannot* discover a larger area if we move the long pointer
            // because the width is decreasing, and the height is limited by the shorter line
            if height[short] < height[long] {
                short += 1;
            } else {
                long -= 1;
            }
        }

        max_area
    }

    fn area(height: &[i32], short: usize, long: usize) -> i32 {
        std::cmp::min(height[long], height[short]) * ((long - short) as i32)
    }
}