use proconio::input;

fn main() {
    input! {
        n: usize
    }

    let mut dp = [0u64; 50];
    dp[0] = 1;
    for i in 0..n {
        dp[i + 1] += dp[i];
        dp[i + 2] += dp[i];
    }
    println!("{}", dp[n]);
}
