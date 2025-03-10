use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let mut dp = vec![i64::MIN; n + 1];

    dp[0] = 0;
    for i in 0..n {
        if i >= 1 {
            dp[i + 1] = dp[i + 1].max(dp[i - 1]);
        }
        dp[i + 1] = dp[i + 1].max(dp[i] + a[i]);
    }

    println!("{}", dp[n]);
}
