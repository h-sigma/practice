impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let mut rows = obstacle_grid.len();
        let mut cols = obstacle_grid[0].len();

        let mut paths = vec![0; cols];

        for r in 0..rows {
            for c in 0..cols {
                // if on obstacle, it cannot be reached at all
                if obstacle_grid[r][c] == 1 {
                    paths[c] = 0;
                    continue;
                }
                // if top left-corner and not obstacle, exactly 1 way to reach
                if r == 0 && c == 0 {
                    paths[c] = 1;
                    continue;
                }

                // save the previous row's "unique paths" value for this column
                let mut up = paths[c];
                paths[c] = 0;
                // if reachable from left, add unique paths of left cell
                if c > 0 && obstacle_grid[r][c - 1] == 0 {
                    paths[c] += paths[c - 1];
                }
                // if reachable from above, add unique paths of upper cell
                if r > 0 && obstacle_grid[r - 1][c] == 0 {
                    paths[c] += up;
                }
            } 
        }

        paths[cols - 1]
    }
}