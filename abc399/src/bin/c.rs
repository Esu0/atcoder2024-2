use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(usize, usize); m],
    }

    let mut cnt = 0;
    let mut stack = Vec::new();
    let mut vis = vec![false; n];
    let mut g = vec![vec![]; n];
    for &(ui, vi) in &uv {
        g[ui - 1].push(vi - 1);
        g[vi - 1].push(ui - 1);
    }
    for u in 0..n {
        if !vis[u] {
            cnt += 1;
            stack.clear();
            stack.push(u);
            vis[u] = true;
            while let Some(u) = stack.pop() {
                for &v in &g[u] {
                    if !vis[v] {
                        vis[v] = true;
                        stack.push(v);
                    }
                }
            }
        }
    }
    println!("{}", m - (n - cnt));
}
