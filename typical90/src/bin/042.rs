use proconio::input;
use util::ModInt;

type MInt = ModInt<1000000007>;

fn main() {
    input! {
        k: usize,
    }
    if k % 9 != 0 {
        println!("0");
    } else {
        let mut dp = vec![MInt::new(0); k + 1];
        dp[0] = MInt::new(1);
        for i in 0..k {
            for j in i + 1..=k.min(i + 9) {
                let tmp = dp[i];
                dp[j] += tmp;
            }
        }
        println!("{}", dp[k]);
    }
}
