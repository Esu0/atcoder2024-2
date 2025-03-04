use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        c: [[u64; n]; n],
        q: usize,
    }

    let g = c;
    let mut dp = vec![vec![u64::MAX; n]; 1 << k];
    // dp[0].fill(0);
    for i in 0..k {
        dp[1 << i][i] = 0;
    }

    let mut queue = BinaryHeap::new();
    let mut buf = vec![0; n];
    for s in 1..1 << k {
        let mut t = s;
        while t > 0 {
            t = (t - 1) & s;
            let u = s ^ t;
            for v in 0..n {
                dp[s][v] = dp[s][v].min(dp[t][v].saturating_add(dp[u][v]));
            }
        }
        queue.clear();
        queue.extend(dp[s].iter().enumerate().map(|(v, &c)| (Reverse(c), v)));
        buf.fill(u64::MAX);
        while let Some((Reverse(c), u)) = queue.pop() {
            if buf[u] != u64::MAX {
                continue;
            }
            buf[u] = c;
            for v in 0..n {
                if buf[v] == u64::MAX {
                    queue.push((Reverse(c + g[u][v]), v));
                }
            }
        }
        dp[s].clone_from_slice(&buf);
    }

    eprintln!("{:?}", dp[(1 << k) - 1]);
    for _ in 0..q {
        input! { s: usize, t: usize }
        
    }
}
