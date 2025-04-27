use std::cmp::Reverse;

use proconio::input;

fn main() {
    input! {
        n: usize,
        uv: [(usize, usize); n - 1],
    }

    let mut adj_list = vec![vec![]; n];
    for &(u, v) in &uv {
        adj_list[u - 1].push(v - 1);
        adj_list[v - 1].push(u - 1);
    }
    let degrees = adj_list.iter().map(|x| x.len()).collect::<Vec<_>>();
    for adju in &mut adj_list {
        adju.sort_unstable_by_key(|&v| Reverse(degrees[v]));
    }
    let mut ans = 0;
    for adju in &adj_list {
        let mut min_deg = usize::MAX;
        for (i, &v) in adju.iter().enumerate() {
            min_deg = min_deg.min(degrees[v]);
            ans = ans.max(1 + i + 1 + (i + 1) * (min_deg - 1));
        }
    }
    println!("{}", n - ans);
}
