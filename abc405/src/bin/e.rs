use proconio::input;
type MInt = util::ModInt<998244353>;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    }

    let mut fact = [MInt::new(1); 3_000_001];
    let mut facti = fact;
    // println!("{}", fact.len());
    for i in 1..fact.len() {
        fact[i] = fact[i - 1] * MInt::new(i as _);
        // println!("jfaksd");
        facti[i] = fact[i].inv();
    }
    let n = a + b + c + d;
    let mut ans = MInt::new(0);
    for l in d..=c + d {
        ans += fact[n - l] * facti[b] * facti[n - l - b] * fact[l - 1] * facti[d - 1] * facti[l - d];
    }
    println!("{ans}");
}
