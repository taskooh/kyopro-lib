pub trait RerootingData {
    type Cost: Copy + num_traits::One;
    type Data: Copy;

    fn merge(&self, first: Self::Data, second: Self::Data) -> Self::Data;
    // how to use children information to update current node.
    fn apply(&self, value: Self::Data, index: usize, parent: usize, cost: Self::Cost)
        -> Self::Data;
    // e() is identity element of merge.
    // merge(a,e()) = merge(e(),a) = a
    fn e(&self) -> Self::Data;
    fn leaf(&self) -> Self::Data;
}

pub struct Rerooting<T: RerootingData> {
    n: usize,
    g: Vec<Vec<(usize, T::Cost)>>,
    memo: Vec<T::Data>,
    ans: Vec<T::Data>,
    d: T,
}

impl<T: RerootingData> Rerooting<T> {
    pub fn new_from_graph(g: Vec<Vec<usize>>, d: T) -> Self {
        let n = g.len();
        let g2 = g
            .iter()
            .map(|v| {
                v.iter()
                    .map(|&j| (j, <T::Cost as num_traits::One>::one()))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        Self {
            n,
            g: g2,
            memo: vec![d.e(); n],
            ans: vec![d.e(); n],
            d,
        }
    }

    pub fn new_from_graph_with_cost(g: Vec<Vec<(usize, T::Cost)>>, d: T) -> Self {
        let n = g.len();
        Self {
            n,
            g,
            memo: vec![d.e(); n],
            ans: vec![d.e(); n],
            d,
        }
    }

    fn dfs1(&mut self, current: usize, parent: usize) {
        let mut upd = false;
        for (next, next_cost) in self.g[current].clone() {
            if next == parent {
                continue;
            }
            self.dfs1(next, current);
            upd = true;
            self.memo[current] = self.d.merge(
                self.memo[current],
                self.d.apply(self.memo[next], next, current, next_cost),
            )
        }
        if !upd {
            self.memo[current] = self.d.leaf();
        }
    }

    fn dfs2(&mut self, current: usize, parent: usize, v: T::Data) {
        let mut to_child: Vec<T::Data> = vec![];
        for &(next, next_cost) in &self.g[current] {
            if next == parent {
                to_child.push(v);
            } else {
                to_child.push(self.d.apply(self.memo[next], next, current, next_cost));
            }
        }
        // 先頭と末尾からの累積merge
        let mut head = vec![self.d.e(); to_child.len() + 1];
        let mut tail = vec![self.d.e(); to_child.len() + 1];
        for i in 0..to_child.len() {
            head[i + 1] = self.d.merge(head[i], to_child[i]);
        }
        for i in (0..to_child.len()).rev() {
            tail[i] = self.d.merge(tail[i + 1], to_child[i]);
        }
        self.ans[current] = *head.last().unwrap();
        // 自身のindexを除いたものをmerge
        for i in 0..self.g[current].len() {
            let (next, next_cost) = self.g[current][i];
            if next == parent {
                continue;
            }
            let next_v = self.d.merge(head[i], tail[i + 1]);
            self.dfs2(
                next,
                current,
                self.d.apply(next_v, current, next, next_cost),
            );
        }
    }

    pub fn run(&mut self, start: usize) -> Vec<T::Data> {
        self.memo = vec![self.d.e(); self.n];
        self.ans = vec![self.d.e(); self.n];
        self.dfs1(start, self.n);
        self.dfs2(start, self.n, self.d.e());
        return self.ans.clone();
    }
}

#[cfg(test)]
mod test {
    use std::cmp::max;

    #[test]
    fn test_rerooting_farthest_path() {
        use crate::rerooting::RerootingData;
        use crate::rerooting::*;
        struct Data {}
        impl RerootingData for Data {
            type Cost = usize;
            type Data = usize;
            fn merge(&self, first: Self::Data, second: Self::Data) -> Self::Data {
                max(first, second)
            }
            fn apply(&self, value: Self::Data, _: usize, _: usize, cost: Self::Cost) -> Self::Data {
                value + cost
            }
            fn e(&self) -> Self::Data {
                0
            }
            fn leaf(&self) -> Self::Data {
                0
            }
        }
        //      0
        //     / \
        //    1   2
        //   /
        //  3
        let g = vec![vec![1, 2], vec![0, 3], vec![0], vec![1]];
        let mut rerooting = Rerooting::new_from_graph(g, Data {});
        let ans = rerooting.run(0);
        assert_eq!(ans, vec![2, 2, 3, 3]);
    }
}
