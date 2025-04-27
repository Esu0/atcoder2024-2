use proconio::input;

fn dfs1(g: &[Vec<usize>], dist: &mut [usize], u: usize) {
    dist[u] = 0;
    let mut mx = 0;
    for &v in &g[u] {
        if dist[v] == usize::MAX {
            dfs1(g, dist, v);
            mx = mx.max(dist[v] + 1);
        }
    }
    dist[u] = mx;
}

fn dfs2(g: &[Vec<usize>], vis: &mut [bool], dist: &mut [usize], u: usize, pd: usize) {
    vis[u] = true;
    let mut mx = 0;
    let mut buf = Vec::with_capacity(g[u].len());
    for &v in &g[u] {
        if !vis[v] {
            buf.push(mx);
            mx = mx.max(dist[v] + 1);
        }
    }
    mx = 0;
    let mut i = buf.len();
    for &v in g[u].iter().rev() {
        if !vis[v] {
            i -= 1;
            let mx_nxt = mx.max(dist[v] + 1);
            dfs2(g, vis, dist, v, pd.max(mx.max(buf[i])) + 1);
            mx = mx_nxt;
        }
    }
    dist[u] = mx.max(pd);
}

fn main() {
    input! {
        n1: usize,
        uv1: [(usize, usize); n1 - 1],
        n2: usize,
        uv2: [(usize, usize); n2 - 1],
    }

    let mut dist1 = vec![usize::MAX; n1];
    let mut dist2 = vec![usize::MAX; n2];

    let mut g1 = vec![vec![]; n1];
    let mut g2 = vec![vec![]; n2];
    for &(ui, vi) in &uv1 {
        g1[ui - 1].push(vi - 1);
        g1[vi - 1].push(ui - 1);
    }
    for &(ui, vi) in &uv2 {
        g2[ui - 1].push(vi - 1);
        g2[vi - 1].push(ui - 1);
    }

    let mut vis = vec![false; n1];
    dfs1(&g1, &mut dist1, 0);
    // eprintln!("{dist1:?}");
    dfs2(&g1, &mut vis, &mut dist1, 0, 0);
    let r1 = dist1.iter().copied().max().unwrap();
    let mut vis = vec![false; n2];
    dfs1(&g2, &mut dist2, 0);
    // eprintln!("{dist2:?}");
    dfs2(&g2, &mut vis, &mut dist2, 0, 0);
    let r2 = dist2.iter().copied().max().unwrap();
    // eprintln!("{dist1:?}");
    // eprintln!("{dist2:?}");

    dist1.sort_unstable();
    let mut cumsum1 = Vec::with_capacity(dist1.len() + 1);
    cumsum1.push(0);
    let mut s = 0;
    for &d1 in &dist1 {
        s += d1;
        cumsum1.push(s);
    }

    let r = r1.max(r2);
    let mut ans = 0;
    for &d2 in &dist2 {
        let k = dist1.partition_point(|&x| x + d2 + 1 < r);
        ans += r * k + cumsum1[n1] - cumsum1[k] + (d2 + 1) * (n1 - k);
    }
    println!("{ans}");
}
