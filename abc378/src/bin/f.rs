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

    let mut vis = vec![false; n];
    let mut stack = Vec::with_capacity(n);
    let mut ans = 0;
    for i in 0..n {
        if !vis[i] && g[i].len() == 3 {
            let mut cnt = 0usize;
            stack.clear();
            stack.push(i);
            vis[i] = true;
            while let Some(u) = stack.pop() {
                for &v in &g[u] {
                    if g[v].len() == 2 {
                        cnt += 1;
                    }
                    if !vis[v] && g[v].len() == 3 {
                        vis[v] = true;
                        stack.push(v);
                    }
                }
            }
            if cnt > 0 {
                ans += cnt * (cnt - 1) / 2;
            }
        }
    }
    println!("{ans}");
}
