pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![1; n],
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        let mut x = x;
        while x != self.parent[x] {
            self.parent[x] = self.parent[self.parent[x]];
            x = self.parent[x];
        }
        x
    }

    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x == root_y {
            return false;
        }
        if self.rank[root_x] > self.rank[root_y] {
            self.parent[root_y] = root_x;
            self.rank[root_x] += self.rank[root_y];
        } else {
            self.parent[root_x] = root_y;
            self.rank[root_y] += self.rank[root_x];
        }

        true
    }
}

pub fn find_critical_and_pseudo_critical_edges(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = n as usize;
    let mut edges = edges
        .iter()
        .enumerate()
        .map(|(i, edge)| {
            let mut edge = edge.clone();
            edge.push(i as i32);
            edge
        })
        .collect::<Vec<_>>();
    edges.sort_unstable_by_key(|edge| edge[2]);

    // Calculate MST weight first
    let mut uf = UnionFind::new(n);
    let mut mst_weight = 0;

    for edge in &edges {
        if uf.union(edge[0] as usize, edge[1] as usize) {
            mst_weight += edge[2];
        }
    }

    let (mut critical, mut pseudo_critical) = (Vec::new(), Vec::new());
    for edge in &edges {
        let (n1, n2, e_weight, i) = (edge[0], edge[1], edge[2], edge[3]);
        let mut weight = 0;
        let mut uf = UnionFind::new(n);
        for edge in &edges {
            let (v1, v2, w, j) = (edge[0], edge[1], edge[2], edge[3]);
            if i != j && uf.union(v1 as usize, v2 as usize) {
                weight += w;
            }
        }
        if *uf.rank.iter().max().unwrap() != n || weight > mst_weight {
            critical.push(i);
            continue;
        }

        let mut uf = UnionFind::new(n);
        weight = e_weight;
        uf.union(n1 as usize, n2 as usize);
        for edge in &edges {
            let (v1, v2, w, j) = (edge[0], edge[1], edge[2], edge[3]);
            if i != j && uf.union(v1 as usize, v2 as usize) {
                weight += w;
            }
        }
        if weight == mst_weight {
            pseudo_critical.push(i);
        }
    }

    vec![critical, pseudo_critical]
}

#[test]
fn test_find_critical_and_pseudo_critical_edges() {
    let n = 6;
    let edges = vec![
        vec![0, 1, 1],
        vec![1, 2, 1],
        vec![0, 2, 1],
        vec![2, 3, 4],
        vec![3, 4, 2],
        vec![3, 5, 2],
        vec![4, 5, 2],
    ];
    let res = vec![vec![3], vec![0, 1, 2, 4, 5, 6]];
    assert_eq!(find_critical_and_pseudo_critical_edges(n, edges), res);
}
