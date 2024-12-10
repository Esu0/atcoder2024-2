use proconio::input;
use util::ModInt;
type MInt = ModInt<998244353>;

fn main() {
    input! {
        n: usize,
        a: [u8; n],
    }
    let mut b = vec![];
    let mut count = 1usize;
    let mut prev = a[0];
    for &ai in &a[1..] {
        if prev != ai {
            b.push((prev, count));
            count = 1;
            prev = ai;
        } else {
            count += 1;
        }
    }
    b.push((prev, count));
    // eprintln!("{b:?}");
    if b[0].0 != 1 || b.iter().any(|&(_, c)| c % 2 == 0) {
        println!("0");
        return;
    }

    let mut frac2 = vec![MInt::new(1); n + 1];
    for i in 3..=n {
        if i % 2 == 1 {
            let tmp = frac2[i - 2] * MInt::new(i as _);
            frac2[i] = tmp;
        }
    }
    let mut frac = vec![MInt::new(1); n + 1];
    for i in 1..=n {
        let tmp = frac[i - 1] * MInt::new(i as _);
        frac[i] = tmp;
    }

    let c = b.iter().map(|&(_, c)| if c == 1 { MInt::new(1) } else { frac2[c - 2] }).collect::<Vec<_>>();
    let mut ans = MInt::new(1);
    let mut cur_l = 0;
    for &(_, ci) in &b {
        if ci == 1 { continue; }
        let l = ci / 2;
        ans *= frac[l + cur_l] * frac[l].inv() * frac[cur_l].inv();
        cur_l += l;
    }
    ans *= c.iter().copied().fold(MInt::new(1), |acc, x| acc * x);
    println!("{ans}");
}
