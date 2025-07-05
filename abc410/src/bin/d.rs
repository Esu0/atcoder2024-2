use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        abw: [(usize, usize, usize); m],
    }

    let mut vis = vec![[false; 1 << 10]; n];
    let mut g = vec![vec![]; n];

    for &(ai, bi, wi) in &abw {
        g[ai - 1].push((bi - 1, wi));
    }

    let mut stack = Vec::with_capacity(n * (1 << 10));
    stack.push((0, 0));
    vis[0][0] = true;

    while let Some((u, val)) = stack.pop() {
        for &(v, w) in &g[u] {
            let nval = val ^ w;
            if vis[v][nval] {
                continue;
            }
            vis[v][nval] = true;
            stack.push((v, nval));
        }
    }

    for (ans, &cond) in vis[n - 1].iter().enumerate() {
        if cond {
            println!("{ans}");
            return;
        }
    }
    println!("-1");
}
