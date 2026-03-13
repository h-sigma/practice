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

    pub fn count(&self) -> usize {
        self.roots().count()
    }

    pub fn roots(&self) -> impl Iterator<Item = usize> {
        self.parents.iter().enumerate().filter_map(|(i, p)| {
            if i == *p {
                Some(*p)
            } else {
                None
            }
        })
    }
}
