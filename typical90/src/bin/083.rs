use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(usize, usize); m],
    }

    let mut g = vec![vec![]; n];
    let mut g_lazy = vec![vec![]; n];
    for &(u, v) in &uv {
        g[u - 1].push(v - 1);
        g[v - 1].push(u - 1);
    }

    let b = 600usize;

    for &(u, v) in &uv {
        let u = u - 1;
        let v = v - 1;
        if g[u].len() >= b {
            g_lazy[v].push(u);
        }
        if g[v].len() >= b {
            g_lazy[u].push(v);
        }
    }

    let mut lazy = vec![(1, 0); n];
    let mut col = vec![(1, 0); n];

    input! { q: usize }

    for i in 1..=q {
        input! { x: usize, y: u32 }
        let x = x - 1;
        let mut ans = col[x];
        for &v in &g_lazy[x] {
            ans = std::cmp::max_by_key(ans, lazy[v], |&(_, t)| t);
        }
        println!("{}", ans.0);
        col[x] = (y, i);
        if g[x].len() >= b {
            lazy[x] = (y, i);
        } else {
            for &v in &g[x] {
                col[v] = (y, i);
            }
        }
    }
}
