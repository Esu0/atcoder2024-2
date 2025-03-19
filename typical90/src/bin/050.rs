use proconio::input;
use util::ModInt;
type MInt = ModInt<1000000007>;

macro_rules! chsum {
    ($d:expr, $s:expr) => {{
        let tmp = $d + $s;
        $d = tmp;
    }};
}
fn main() {
    input! {
        n: usize,
        l: usize,
    }

    let mut dp = vec![MInt::new(0); n + 1];
    dp[0] = MInt::new(1);
    for i in 0..n {
        chsum!(dp[i + 1], dp[i]);
        let j = i + l;
        if j <= n {
            chsum!(dp[j], dp[i]);
        }
    }
    println!("{}", dp[n]);
}
