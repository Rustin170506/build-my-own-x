pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x == root_y {
            return false;
        }

        match self.rank[root_x].cmp(&self.rank[root_y]) {
            std::cmp::Ordering::Less => self.parent[root_x] = root_y,
            std::cmp::Ordering::Greater => self.parent[root_y] = root_x,
            std::cmp::Ordering::Equal => {
                self.parent[root_y] = root_x;
                self.rank[root_x] += 1;
            }
        }

        true
    }

    pub fn reset(&mut self, n: usize) {
        self.parent = (0..n).collect();
        self.rank = vec![0; n];
    }
}

pub fn find_critical_and_pseudo_critical_edges(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = n as usize;
    let mut edges = edges
        .iter()
        .enumerate()
        .map(|(i, edge)| (edge.clone(), i as i32))
        .collect::<Vec<_>>();
    edges.sort_unstable_by_key(|(edge, _)| edge[2]);

    let mut uf = UnionFind::new(n);
    let mut mst_weight = 0;

    for (edge, _) in &edges {
        if uf.union(edge[0] as usize, edge[1] as usize) {
            mst_weight += edge[2];
        }
    }

    let (mut critical, mut pseudo_critical) = (Vec::new(), Vec::new());

    for (edge, i) in &edges {
        // Check if this edge is critical
        let mut uf = UnionFind::new(n);
        let mut weight = 0;
        let mut edges_used = 0;

        for (other_edge, _) in &edges {
            if other_edge == edge {
                continue;
            }
            if uf.union(other_edge[0] as usize, other_edge[1] as usize) {
                weight += other_edge[2];
                edges_used += 1;
            }
        }

        if edges_used == n - 1 && weight != mst_weight {
            critical.push(*i);
            continue;
        }

        // Check if this edge is pseudo-critical
        let mut uf = UnionFind::new(n);
        uf.union(edge[0] as usize, edge[1] as usize);
        weight = edge[2];
        edges_used = 1;

        for (other_edge, _) in &edges {
            if uf.union(other_edge[0] as usize, other_edge[1] as usize) {
                weight += other_edge[2];
                edges_used += 1;
            }
        }

        if edges_used == n - 1 && weight == mst_weight {
            pseudo_critical.push(*i);
        }
    }

    vec![critical, pseudo_critical]
}

#[test]
fn test_find_critical_and_pseudo_critical_edges() {
    let n = 5;
    let edges = vec![
        vec![0, 1, 1],
        vec![1, 2, 1],
        vec![2, 3, 2],
        vec![0, 3, 2],
        vec![0, 4, 3],
        vec![3, 4, 3],
        vec![1, 4, 6],
    ];
    let res = find_critical_and_pseudo_critical_edges(n, edges);
    assert_eq!(res, vec![vec![0, 1], vec![2, 3, 4, 5]]);
}
