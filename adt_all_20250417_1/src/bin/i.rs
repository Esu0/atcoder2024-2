use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        uv: [(usize, usize); n - 1],
    }

    let mut g = vec![vec![]; n];
    for &(ui, vi) in &uv {
        g[ui - 1].push(vi - 1);
        g[vi - 1].push(ui - 1);
    }

    let mut stack = vec![];
    let mut set = HashSet::new();
    let mut vis = vec![false; n];
    let mut ans = 0;
    for u in 0..n {
        if vis[u] || g[u].len() != 3 {
            continue;
        }
        stack.push(u);
        set.clear();
        while let Some(u) = stack.pop() {
            if vis[u] {
                continue;
            }
            vis[u] = true;
            for &v in &g[u] {
                if !vis[v] && g[v].len() == 3 {
                    stack.push(v);
                } else if g[v].len() == 2 {
                    set.insert(v);
                }
            }
        }
        let l = set.len();
        ans += l * (l - 1) / 2;
    }
    println!("{ans}");
}
