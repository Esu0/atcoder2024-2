use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }

    let mut dp = vec![vec![u64::MAX; n + 1]; n + 1];
    dp[0].fill(0);
    for d in 1..=n {
        for l in 0..=n - d {
            if d % 2 == 0 {
                let mut all_xor = 0;
                for i in 0..d {
                    all_xor ^= a[l + i];
                }
                for i in 1..d {
                    if i % 2 == 0 {
                        dp[d][l] = dp[d][l].min(dp[i][l] + dp[d - i][l + i]);
                    } else {
                        dp[d][l] = dp[d][l].min(dp[i][l] + dp[d - i][l + i] + 2 * all_xor);
                    }
                }
            } else {
                for i in (0..d).step_by(2) {
                    dp[d][l] = dp[d][l].min(dp[i][l] + dp[d - i - 1][l + i + 1]);
                }
            }
        }
    }

    let ans = dp[n][0]
        + if n % 2 == 0 {
            0
        } else {
            a.iter().copied().reduce(|acc, x| acc ^ x).unwrap()
        };
    println!("{ans}");
}
