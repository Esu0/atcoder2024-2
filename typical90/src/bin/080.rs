use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
    }

    let mut b = vec![0; d];
    for i in 0..n {
        input! { ai: usize }
        for (j, bj) in b.iter_mut().enumerate() {
            *bj |= ((ai >> j) & 1) << i;
        }
    }

    let mut dp = vec![0u64; 1 << n];
    let mut dp_nxt = vec![0; 1 << n];
    dp[0] = 1;
    for &bi in &b {
        dp_nxt.clone_from_slice(&dp);
        for j in 0..1 << n {
            dp_nxt[j | bi] += dp[j];
        }
        dp.clone_from_slice(&dp_nxt);
    }

    println!("{}", dp[(1 << n) - 1]);
}
