use proconio::input;
type MInt = util::ModInt::<998244353>;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    }

    let mut dp = vec![MInt::new(0); d + 1];
    dp[b] = MInt::new(1);
    let mut dp_nxt = dp.clone();
    for i in a..c {
        let mut sum = MInt::new(0);
        for j in b..=d {
            dp_nxt[j] = sum;
            sum = (sum + dp[j]) * MInt::new(i as _);
        }
        for j in b..=d {
            dp_nxt[j] += dp[j] * MInt::new(j as _);
        }
        dp.clone_from_slice(&dp_nxt);
        // eprintln!("{dp:?}");
    }
    let mut sum = MInt::new(0);
    for j in b..=d {
        sum = sum * MInt::new(c as _) + dp[j];
    }
    println!("{}", sum);
}
