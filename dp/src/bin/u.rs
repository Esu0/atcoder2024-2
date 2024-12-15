use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [[i64; n]; n],
    }

    let mut dp = vec![0; 1 << n];
    for (i, dpi) in dp.iter_mut().enumerate() {
        for (j, aj) in a.iter().enumerate() {
            for (k, &ajk) in aj.iter().enumerate().skip(j + 1) {
                if i & (1 << j) != 0 && i & (1 << k) != 0 {
                    *dpi += ajk;
                }
            }
        }
    }

    for i in 0..(1 << n) {
        let mut j = i;
        while j > 0 {
            dp[i] = dp[i].max(dp[j] + dp[i & !j]);
            j = (j - 1) & i;
        }
    }
    println!("{}", dp[(1 << n) - 1]);
}
