impl Solution {
    /// Flood fill implementation of DFS.
    /// This is actually faster in rust than the union find implementation, which is surprising to me.
    /// Possibly the input set is too small for the amortization of union-find to kick in?
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut grid = grid;

        let mut m = grid.len();
        let mut n = grid[0].len();

        let mut islands = 0;

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == '1' {
                    Solution::flood_fill(&mut grid, i, j, m, n);
                    islands += 1;
                }
            }
        }

        islands
    }

    pub fn flood_fill(grid: &mut Vec<Vec<char>>, i: usize, j: usize, m: usize, n: usize) {
        if i >= m || j >= n || grid[i][j] == '0' {
            return;
        }

        grid[i][j] = '0';
        Solution::flood_fill(grid, i + 1, j, m, n);
        Solution::flood_fill(grid, i, j + 1, m, n);
        Solution::flood_fill(grid, i - 1, j, m, n);
        Solution::flood_fill(grid, i, j - 1, m, n);
    }
}