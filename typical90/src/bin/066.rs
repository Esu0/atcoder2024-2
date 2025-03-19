use proconio::input;

fn main() {
    input! {
        n: usize,
        lr: [(usize, usize); n],
    }

    let mut dp = vec![vec![0.; 102]; n + 1];
    let mut ans = 0.;
    for (i, &(li, ri)) in lr.iter().enumerate() {
        let mut s = 0.;
        for j in 0..=100 {
            if (li..=ri).contains(&j) {
                s += dp[i][j + 1];
            }
            let diff = if j <= li {
                1.
            } else if j <= ri {
                (ri - j + 1) as f64 / (ri - li + 1) as f64
            } else {
                0.
            };
            dp[i + 1][j] = dp[i][j] + diff;
        }
        ans += s / (ri - li + 1) as f64;
    }
    // eprintln!("{dp:?}");
    println!("{ans}");
}
