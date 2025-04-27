use proconio::input;
type MInt = util::ModInt<998244353>;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [u32; n],
    }

    let mut fact = [MInt::new(1); 20];
    for i in 0..19 {
        fact[i + 1] = fact[i] * MInt::new((i + 1) as _);
    }
    let ifact = fact.map(MInt::inv);

    let mut sums = vec![MInt::new(0); k + 1];
    let mut nxt = vec![MInt::new(0); k + 1];
    let mut ans = MInt::new(0);
    for &ai in &a {
        nxt.fill(MInt::new(0));
        let ai = MInt::new(ai as _);
        let mut paio = MInt::new(1);
        for i in 0..=k {
            let mut pai = MInt::new(1);
            for j in 0..=i {
                nxt[i] += sums[i - j] * pai * fact[i] * ifact[i - j] * ifact[j];
                pai *= ai;
            }
            nxt[i] += paio;
            paio *= ai;
        }
        sums.clone_from_slice(&nxt);
        ans += sums[k];
        // eprintln!("{sums:?}");
    }
    println!("{}", ans);
}
