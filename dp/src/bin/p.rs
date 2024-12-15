use std::{cmp::Reverse, collections::VecDeque};

use proconio::input;
use util::ModInt;
type MInt = ModInt<1000000007>;

fn main() {
    input! {
        n: usize,
        xy: [(usize, usize); n - 1],
    }
    let mut adj_list = vec![vec![]; n];
    for &(x, y) in &xy {
        adj_list[x - 1].push(y - 1);
        adj_list[y - 1].push(x - 1);
    }

    let mut depth = vec![usize::MAX; n];
    let mut queue = VecDeque::from([(0, 0)]);
    while let Some((u, dep)) = queue.pop_back() {
        depth[u] = dep;
        for &v in &adj_list[u] {
            if depth[v] == usize::MAX {
                queue.push_front((v, dep + 1));
            }
        }
    }
    let mut indice = (0..n).collect::<Vec<_>>();
    indice.sort_unstable_by_key(|&i| Reverse(depth[i]));

    // dp[0]: black, dp[1]: white
    let mut dp = [vec![MInt::new(0); n], vec![MInt::new(0); n]];
    for &i in &indice {
        let depi = depth[i];
        dp[0][i] = adj_list[i].iter().filter(|&&j| depth[j] > depi).map(|&j| dp[1][j]).fold(MInt::new(1), |acc, x| acc * x);
        dp[1][i] = adj_list[i].iter().filter(|&&j| depth[j] > depi).map(|&j| dp[0][j] + dp[1][j]).fold(MInt::new(1), |acc, x| acc * x);
    }
    // eprintln!("{depth:?}");

    println!("{}", dp[0][0] + dp[1][0]);
}
