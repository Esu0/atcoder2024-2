use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        r: [[usize]; m],
    }

    let mut g = vec![vec![]; n];
    for (i, ri) in r.iter().enumerate() {
        for &j in ri {
            g[j - 1].push(i);
        }
    }

    let mut dist = vec![usize::MAX; n];
    let mut queue = VecDeque::from([(0, 0)]);
    let mut visited = vec![false; m];
    while let Some((u, d)) = queue.pop_front() {
        if dist[u] != usize::MAX {
            continue;
        }
        dist[u] = d;
        for &v in &g[u] {
            if visited[v] {
                continue;
            }
            visited[v] = true;
            for &x in &r[v] {
                queue.push_back((x - 1, d + 1));
            }
        }
    }

    dist.iter().for_each(|&di| println!("{}", di as isize));
}
