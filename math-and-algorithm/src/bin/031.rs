use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut dp = [0, i64::MIN];
    for _ in 0..n {
        input! { a: i64 }
        dp = [dp[0].max(dp[1]), dp[0] + a];
    }
    println!("{}", dp[0].max(dp[1]));
}
