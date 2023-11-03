use std::cmp::max;

use kyopro_lib::{Rerooting, RerootingData};
use proconio::input;

fn main() {
    input! {
        n: usize,
        a_b_c: [(usize, usize, i64); n-1],
        d: [i64; n],
    }
    let a_b_c = a_b_c
        .iter()
        .map(|&(a, b, c)| (a - 1, b - 1, c))
        .collect::<Vec<_>>();
    let mut g: Vec<Vec<(usize, i64)>> = vec![vec![]; n];
    for &(a, b, c) in &a_b_c {
        g[a].push((b, c));
        g[b].push((a, c));
    }
    let rerooting_data = MyRerootingData { d };
    let mut rr = Rerooting::new_from_graph_with_cost(g, rerooting_data);
    let ans = rr.run(0);
    for i in 0..n {
        println!("{}", ans[i]);
    }
}
struct MyRerootingData {
    d: Vec<i64>,
}

impl RerootingData for MyRerootingData {
    type Cost = i64;
    type Data = i64;

    fn merge(&self, first: i64, second: i64) -> i64 {
        max(first, second)
    }

    fn apply(&self, value: i64, current: usize, _parent: usize, cost: i64) -> i64 {
        max(value, self.d[current]) + cost
    }

    fn leaf(&self) -> i64 {
        0
    }

    fn e(&self) -> i64 {
        0
    }
}
