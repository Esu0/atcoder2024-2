use proconio::input;
type MInt = util::ModInt<998244353>;

fn main() {
    input! {
        n: u64,
    }
    let nm = MInt::new(n as _);
    let mut ans = nm * nm - nm * (nm - MInt::new(1)) * MInt::new(2).inv();

    let mut r = n;
    let mut k = 1;
    while r > 1_000_000 {
        let l = n / (k + 1);
        ans -= MInt::new(k as _) * MInt::new((r - l) as _);
        k += 1;
        r = l;
    }
    for b in 1..=r {
        ans -= MInt::new((n / b) as _);
    }
    println!("{ans}");
}
