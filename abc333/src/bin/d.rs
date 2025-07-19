use proconio::input;

fn dfs<A: std::ops::Deref<Target = [usize]>>(u: usize, g: &[A], dp: &mut [usize]) {
    dp[u] = 0;
    let mut s = 1;
    for &v in &g[u][..] {
        if dp[v] == usize::MAX {
            dfs(v, g, dp);
            s += dp[v];
        }
    }
    dp[u] = s;
}

fn main() {
    input! {
        n: usize,
        uv: [(usize, usize); n - 1],
    }

    let mut dp = vec![usize::MAX; n];
    let mut g = vec![vec![]; n];
    for &(ui, vi) in &uv {
        g[ui - 1].push(vi - 1);
        g[vi - 1].push(ui - 1);
    }
    // eprintln!("{g:?}");
    dfs(0, &g, &mut dp);
    let ans = n - g[0].iter().map(|&v| dp[v]).max().unwrap();
    assert_eq!(dp[0], n);
    println!("{ans}");
}
