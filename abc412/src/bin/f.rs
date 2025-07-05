use proconio::input;
type MInt = util::ModInt<998244353>;

fn main() {
    input! {
        n: usize,
        c: usize,
        a: [u32; n],
    }

    let mut ia = a.iter().copied().enumerate().collect::<Vec<_>>();
    ia[c - 1].1 += 1;
    ia.sort_unstable_by_key(|&(_, v)| v);
    let sum = a.iter().copied().sum::<u32>();
    // let sum_inv = MInt::new(sum as _).inv();
    let mut e_cur = MInt::new(0);
    let mut sum_cur = 0;
    for &(i, ai) in ia.iter().rev() {
        sum_cur += ai;
        // e = e_cur * sum_inv + e * (sum - (sum_cur - 1)) / sum + 1
        let t = MInt::new((sum_cur - 1) as _).inv();
        let e = t * e_cur + t * MInt::new(sum as _);
        if i == c - 1 {
            println!("{e}");
            return;
        }
        e_cur += e * MInt::new(ai as _);
    }
}
