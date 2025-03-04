use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        w: [u64; n],
    }

    let mut sum = vec![0; 1 << n];

    for i in 0..n {
        let wi = w[i];
        for s in 0..1 << i {
            sum[s | (1 << i)] = sum[s] + wi;
        }
    }
    let all_sum = sum[(1 << n) - 1];
    sum.iter_mut().for_each(|x| *x = *x * *x);

    let mut dp = vec![vec![u64::MAX; d + 1]; 1 << n];
    dp[0][0] = 0;

    for s in 0..1 << n {
        let mut t = s;
        while t > 0 {
            t = (t - 1) & s;
            // eprintln!("{s:05b} {t:05b}");
            let u = s ^ t;
            for j in 0..d {
                dp[s][j + 1] = dp[s][j + 1].min(dp[t][j].saturating_add(sum[u]));
            }
        }
    }

    let ans = (dp[(1 << n) - 1][d] * d as u64 - all_sum * all_sum) as f64 / (d * d) as f64;
    println!("{ans}");
}
