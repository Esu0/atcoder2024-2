use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        uvw: [(usize, usize, u64); m],
    }

    let mut mat = vec![vec![u64::MAX; n]; n];

    for &(ui, vi, wi) in &uvw {
        mat[ui - 1][vi - 1] = wi;
        mat[vi - 1][ui - 1] = wi;
    }

    let mut dp = vec![vec![vec![]; 1 << n]; n];
    dp[0][1].push(0);

    for s in 1..1 << n {
        for i in 0..n {
            if s & (1 << i) != 0 {
                for j in 0..n {
                    if s & (1 << j) == 0 && mat[i][j] != u64::MAX {
                        let t = dp[i][s].clone();
                        dp[j][s | (1 << j)].extend(t.into_iter().map(|x| x ^ mat[i][j]));
                    }
                }
            }
        }
    }
    let mut ans = u64::MAX;
    for s in 1..1 << n {
        ans = ans.min(dp[n - 1][s].iter().copied().min().unwrap_or(u64::MAX));
    }
    println!("{}", ans);
}
