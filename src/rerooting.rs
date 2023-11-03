pub trait RerootingData {
    type Cost: Copy + num_traits::One;
    type Data: Copy;

    fn merge(first: Self::Data, second: Self::Data) -> Self::Data;
    // how to use children information to update current node.
    fn apply(value: Self::Data, index: usize, parent: usize, cost: Self::Cost) -> Self::Data;
    // e() is identity element of merge.
    // merge(a,e()) = merge(e(),a) = a
    fn e() -> Self::Data;
    fn leaf() -> Self::Data;
}

pub struct Rerooting<T: RerootingData> {
    n: usize,
    g: Vec<Vec<(usize, T::Cost)>>,
    memo: Vec<T::Data>,
    ans: Vec<T::Data>,
}

impl<T: RerootingData> Rerooting<T> {
    pub fn new_from_graph(g: Vec<Vec<usize>>) -> Self {
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
            memo: vec![T::e(); n],
            ans: vec![T::e(); n],
        }
    }

    pub fn new_from_graph_with_cost(g: Vec<Vec<(usize, T::Cost)>>) -> Self {
        let n = g.len();
        Self {
            n,
            g,
            memo: vec![T::e(); n],
            ans: vec![T::e(); n],
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
            self.memo[current] = T::merge(
                self.memo[current],
                T::apply(self.memo[next], next, current, next_cost),
            )
        }
        if !upd {
            self.memo[current] = T::leaf();
        }
    }

    fn dfs2(&mut self, current: usize, parent: usize, v: T::Data) {
        let mut to_child: Vec<T::Data> = vec![];
        for &(next, next_cost) in &self.g[current] {
            if next == parent {
                to_child.push(v);
            } else {
                to_child.push(T::apply(self.memo[next], next, current, next_cost));
            }
        }
        // 先頭と末尾からの累積merge
        let mut head = vec![T::e(); to_child.len() + 1];
        let mut tail = vec![T::e(); to_child.len() + 1];
        for i in 0..to_child.len() {
            head[i + 1] = T::merge(head[i], to_child[i]);
        }
        for i in (0..to_child.len()).rev() {
            tail[i] = T::merge(tail[i + 1], to_child[i]);
        }
        self.ans[current] = *head.last().unwrap();
        // 自身のindexを除いたものをmerge
        for i in 0..self.g[current].len() {
            let (next, next_cost) = self.g[current][i];
            if next == parent {
                continue;
            }
            let next_v = T::merge(head[i], tail[i + 1]);
            self.dfs2(next, current, T::apply(next_v, current, next, next_cost));
        }
    }

    pub fn run(&mut self, start: usize) -> Vec<T::Data> {
        self.memo = vec![T::e(); self.n];
        self.ans = vec![T::e(); self.n];
        self.dfs1(start, self.n);
        self.dfs2(start, self.n, T::e());
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
            fn merge(first: Self::Data, second: Self::Data) -> Self::Data {
                max(first, second)
            }
            fn apply(value: Self::Data, _: usize, _: usize, cost: Self::Cost) -> Self::Data {
                value + cost
            }
            fn e() -> Self::Data {
                0
            }
            fn leaf() -> Self::Data {
                0
            }
        }
        //      0
        //     / \
        //    1   2
        //   /
        //  3
        let g = vec![vec![1, 2], vec![0, 3], vec![0], vec![1]];

        let mut rerooting = Rerooting::<Data>::new_from_graph(g);
        let ans = rerooting.run(0);
        assert_eq!(ans, vec![2, 2, 3, 3]);
    }
}
