use std::collections::VecDeque;

#[derive(Debug)]
struct AdjacencyList {
    al: Vec<Vec<usize>>,
}

impl AdjacencyList {
    pub fn new(size: usize, edges: Vec<Vec<i32>>) -> Self {
        let mut al = vec![vec![]; size];

        for edge in edges {
            al[edge[1] as usize].push(edge[0] as usize); 
        }

        AdjacencyList {
            al
        }
    }

    pub fn calculate_indegrees(&self) -> Vec<usize> {
        let mut indegree_count = std::vec![0; self.al.len()];

        for i in 0..(self.al.len()) {
            for edge in &self.al[i] {
                indegree_count[*edge as usize] += 1;
            }
        }

        return indegree_count;
    }

    pub fn edges(&self, node: usize) -> &[usize] {
        &self.al[node]
    }
}

enum TopologicalSort {
    Sorted { sorted: Vec<usize> },
    Failed { sorted: Vec<usize>, unsorted: Vec<usize> }
}

impl TopologicalSort {
    pub fn from_adjacency_list(adj: &AdjacencyList) -> Self {
        // Kahn's Algorithm
        let mut indegree_count = adj.calculate_indegrees();
        let mut sorted: Vec<usize> = vec![];
        let mut to_process: VecDeque<usize> = VecDeque::new();

        // gather starting points: nodes with 0 indegree
        for i in 0..indegree_count.len() {
            if indegree_count[i] == 0 {
                to_process.push_back(i);
            }
        }

        while let Some(node) = to_process.pop_front() {
            sorted.push(node);
            for edge in adj.edges(node) {
                let edge = *edge as usize;
                indegree_count[edge] -= 1;
                if indegree_count[edge] == 0 {
                    to_process.push_back(edge);
                }
            }
        }

        let unsorted = indegree_count.into_iter().enumerate().filter(|(node, indegree)| *indegree > 0).map(|(node, _)| node).collect::<Vec<_>>();

        match unsorted.len() {
            0 => TopologicalSort::Sorted { sorted },
            _ => TopologicalSort::Failed { sorted, unsorted },
        }
    }
}

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let adjacency = AdjacencyList::new(num_courses as usize, prerequisites);
        let result = TopologicalSort::from_adjacency_list(&adjacency);

        match result {
            TopologicalSort::Sorted{sorted} => true,
            TopologicalSort::Failed{sorted, unsorted} => false,
        }
    }
}