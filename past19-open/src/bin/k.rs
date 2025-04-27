use proconio::input;

fn dfs(dp: &mut [Vec<[i32; 2]>], g: &[Vec<usize>], a: &[i32], k: usize, u: usize) {
    dp[u].extend((0..=k).map(|_| [i32::MIN, i32::MIN]));
    dp[u][0][0] = 0;
    dp[u][0][1] = 0;
    for &v in &g[u] {
        if dp[v].is_empty() {
            dfs(dp, g, a, k, v);
            for i in (0..=k).rev() {
                for j in 0..=i {
                    dp[u][i][0] = dp[u][i][0].max(dp[u][i - j][0].saturating_add(dp[v][j][0].max(dp[v][j][1])));
                    dp[u][i][1] = dp[u][i][1].max(dp[u][i - j][1].saturating_add(dp[v][j][0]));
                }
            }
        }
    }
    for i in (0..k).rev() {
        dp[u][i + 1][1] = dp[u][i][1] + a[u];
    }
    dp[u][0][1] = i32::MIN;
}

fn main() {
    input! {
        n: usize,
        k: usize,
        uv: [(usize, usize); n - 1],
        a: [i32; n],
    }

    let mut g = vec![vec![]; n];

    for &(ui, vi) in &uv {
        g[ui - 1].push(vi - 1);
        g[vi - 1].push(ui - 1);
    }

    let mut dp = (0..n).map(|_| Vec::with_capacity(k + 1)).collect::<Vec<_>>();

    dfs(&mut dp, &g, &a, k, 0);
    let ans = dp[0][k][0].max(dp[0][k][1]);
    if ans < 0 {
        println!("-1");
    } else {
        println!("{ans}");
    }
}
