use std::{cmp::max, sync::Mutex};

use kyopro_lib::{Rerooting, RerootingData};
use proconio::input;

// create global variables d: Vec<i64>.
lazy_static::lazy_static! {
    static ref D: Mutex<Vec<i64>> = Mutex::new(vec![]);
}

fn main() {
    input! {
        n: usize,
        a_b_c: [(usize, usize, i64); n-1],
        d_input: [i64; n],
    }
    // set global variables D <- d_input.
    {
        let mut d = D.lock().unwrap();
        *d = d_input;
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
    let mut rr = Rerooting::<MyRerootingData>::new_from_graph_with_cost(g);
    let ans = rr.run(0);
    for i in 0..n {
        println!("{}", ans[i]);
    }
}
struct MyRerootingData;

impl RerootingData for MyRerootingData {
    type Cost = i64;
    type Data = i64;

    fn merge(first: i64, second: i64) -> i64 {
        max(first, second)
    }

    fn apply(value: i64, current: usize, _parent: usize, cost: i64) -> i64 {
        let d = D.lock().unwrap();
        max(value, d[current]) + cost
    }

    fn leaf() -> i64 {
        0
    }

    fn e() -> i64 {
        0
    }
}
