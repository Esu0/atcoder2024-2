use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            a: [u64; n],
        }

        let mut dp = vec![u64::MAX; n + 1];
        dp[0] = 0;
        for i in 0..n {
            dp[i + 1] = dp[i + 1].min(dp[i] + a[i]);
            if i + 2 <= n {
                dp[i + 2] = dp[i + 2].min(dp[i] + a[i].max(a[i + 1]));
            }
        }
        println!("{}", dp[n]);
    }
}
