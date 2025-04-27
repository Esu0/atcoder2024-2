use proconio::input;

fn dfs(dp: &mut [Vec<f64>], x: usize, solved: usize, problems: &[(u64, usize, u64)]) -> f64 {
    if dp[solved][x].is_finite() {
        return dp[solved][x];
    }

    let mut v = 0.0f64;
    for (i, &(si, ci, pi)) in problems.iter().enumerate() {
        if (solved >> i) & 1 != 0 {
            continue;
        }
        if x < ci {
            continue;
        }
        let p1 = pi as f64 / 100.;
        let p2 = (100 - pi) as f64 / 100.;
        let tmp = dfs(dp, x - ci, solved, problems) * p2 + (dfs(dp, x - ci, solved | (1 << i), problems) + si as f64) * p1;
        v = v.max(tmp);
    }

    dp[solved][x] = v;
    v
}

fn main() {
    input! {
        n: usize,
        x: usize,
        scp: [(u64, usize, u64); n],
    }

    let mut dp = vec![vec![f64::INFINITY; x + 1]; 1 << n];
    println!("{}", dfs(&mut dp, x, 0, &scp));
}
