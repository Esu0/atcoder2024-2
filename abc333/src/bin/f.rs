use proconio::input;
type MInt = util::ModInt<998244353>;

fn main() {
    input! {
        n: usize,
    }

    let mut fact = [MInt::new(1); 3000];
    let mut ifact = fact;
    for i in 1..fact.len() {
        fact[i] = MInt::new(i as _) * fact[i - 1];
        ifact[i] = fact[i].inv();
    }

    let mut dp = vec![MInt::new(1); n];

    let mut p2k = MInt::new(2);
    for k in 1..n {
        let mut s = MInt::new(0);
        for i in 0..k {
            s += fact[k] * ifact[i] * ifact[k - i] * dp[i];
        }
        p2k *= MInt::new(2);
        dp[k] = s * (p2k - MInt::new(1)).inv();
    }

    print!("{}", dp[n - 1]);
    let inv2 = MInt::new(2).inv();
    let mut p2ii = MInt::new(1);
    for i in 1..n {
        let mut ans = MInt::new(0);
        for j in 0..=i {
            ans += fact[i] * ifact[j] * ifact[i - j] * dp[n - j - 1];
        }
        p2ii *= inv2;
        print!(" {}", ans * p2ii);
    }
    println!();
}
