use std::cmp::Reverse;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        x: u64,
        uv: [(usize, usize); m],
    }

    let mut g = vec![vec![]; n * 2];

    for &(ui, vi) in &uv {
        let ui = ui - 1;
        let vi = vi - 1;
        g[ui].push((vi, 1));
        g[vi + n].push((ui + n, 1));
    }

    for i in 0..n {
        g[i].push((i + n, x));
        g[i + n].push((i, x));
    }

    let mut queue = std::collections::BinaryHeap::from([(Reverse(0), 0)]);
    let mut dist = vec![u64::MAX; n * 2];
    while let Some((Reverse(d), u)) = queue.pop() {
        if dist[u] != u64::MAX {
            continue;
        }
        dist[u] = d;
        for &(v, e) in &g[u] {
            if dist[v] == u64::MAX {
                queue.push((Reverse(d + e), v));
            }
        }
    }

    // eprintln!("{dist:?}");
    println!("{}", dist[n - 1].min(dist[n * 2 - 1]));
}
