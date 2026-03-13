#[derive(Debug)]
struct DisjointSet {
    parents: Vec<usize>,
    rank: Vec<usize>
}

impl DisjointSet {
    pub fn new(components: usize) -> DisjointSet {
        let mut parents = vec![0; components];
        let rank = vec![1; components]; // each set contains 1 element at the start

        for i in 0..components {
            parents[i] = i; // each component's parent is its own set at the start
        }

        DisjointSet {
            parents,
            rank
        }
    }

    pub fn find(&mut self, component: usize) -> usize {
        if self.parents[component] == component {
            // found root
            component
        } else {
            let root = self.find(self.parents[component]);
            self.parents[component] = root; // path compression
            root
        }
    }

    pub fn union(&mut self, a: usize, b: usize) -> bool {
        let root_a = self.find(a);
        let root_b = self.find(b);

        if root_a == root_b {
            return false; // already in same set
        }

        let (big, smol) = if self.rank[root_a] > self.rank[root_b] { (root_a, root_b) } else { (root_b, root_a) };
        self.parents[smol] = big;
        self.rank[big] += self.rank[smol];
        true
    }
}

impl Solution {
    pub fn num_land_pieces(grid: Vec<Vec<char>>) -> i32 {
        let mut grid = grid;

        let mut m = grid.len();
        let mut n = grid[0].len();

        let mut set = DisjointSet::new(m * n);

        let coords_to_idx = |i, j| i * n + j;
        let idx_to_coords = |idx: usize| (idx / n, idx % n);

        //  keep track of individual islands
        let mut land_pieces = 0;

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] != '1' {
                    continue;
                }
                land_pieces += 1;
                // since we're iterating from top-left corner, we only need to check for union to our left and above
                if i > 0 && grid[i - 1][j] == '1' {
                    if(set.union(coords_to_idx(i, j), coords_to_idx(i - 1, j))) {
                        land_pieces -= 1;
                    }
                }
                if j > 0 && grid[i][j - 1] == '1' {
                    if(set.union(coords_to_idx(i, j), coords_to_idx(i, j - 1))) {
                        land_pieces -= 1;
                    }
                }
            }
        }

        land_pieces
    }
}