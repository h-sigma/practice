impl Solution {
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let mut rot_time = vec![0; m * n];

        for i in 0..m {
            for j in 0..n {
                rot_time[Solution::coords_to_idx(i, j, n)] = match grid[i][j] {
                    0 => 0,
                    _ => i32::MAX,
                };
                // using i32::MAX as sentinel value instead of -1 because we want to use min function to update the rot time, and -1 would mess with that. We can just check for i32::MAX at the end to determine if there are any fresh oranges that never got rotten.
            }
        }

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 2 {
                    Solution::rot_fill(&grid, i, j, m, n, &mut rot_time, 0);
                }
            }
        }

        // if there are any fresh oranges that never got rotten, return -1
        if rot_time.iter().any(|r| *r == i32::MAX) {
            return -1;
        }

        //  the time it takes for all oranges to rot is the maximum rot time among all oranges
        rot_time.iter().max().unwrap().to_owned()
    }

    fn coords_to_idx(i: usize, j: usize, width: usize) -> usize {
        i * width + j
    }

    // flood fill to determine the rot time of each orange. If the rot time is stabilized, i.e. doesn't decrease any further, we can stop the recursion.
    pub fn rot_fill(grid: &Vec<Vec<i32>>, i: usize, j: usize, m: usize, n: usize, rot_time: &mut Vec<i32>, dist: i32) {
        if i >= m || j >= n || grid[i][j] == 0 {
            return;
        }

        let idx = Solution::coords_to_idx(i, j, n);
        let prev_rot = rot_time[idx];
        rot_time[idx] = std::cmp::min(prev_rot, dist);

        if prev_rot == rot_time[idx] {
            // stabilized
            return;
        }

        Solution::rot_fill(grid, i + 1, j, m, n, rot_time, dist + 1);
        Solution::rot_fill(grid, i, j + 1, m, n, rot_time, dist + 1);
        Solution::rot_fill(grid, i - 1, j, m, n, rot_time, dist + 1);
        Solution::rot_fill(grid, i, j - 1, m, n, rot_time, dist + 1);
    }
}