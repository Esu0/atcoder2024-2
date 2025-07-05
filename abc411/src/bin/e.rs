use proconio::input;
type MInt = util::ModInt<998244353>;

fn main() {
    input! {
        n: usize,
        a: [[u32; 6]; n],
    }

    let mut cnt = vec![6u8; n];
    let mut inv = [MInt::new(0); 7];
    for (i, dst) in inv.iter_mut().enumerate().skip(1) {
        *dst = MInt::new(i as _).inv();
    }
    let mut ans = MInt::new(0);
    let mut p = MInt::new(1);

    let mut inds = Vec::with_capacity(n * 6);
    for (i, ai) in a.iter().enumerate() {
        for &aij in ai {
            inds.push((aij, i));
        }
    }
    inds.sort_unstable_by_key(|&(v, _)| std::cmp::Reverse(v));
    for &(v, i) in &inds {
        ans += p * MInt::new(v as _) * inv[cnt[i] as usize];
        p *= MInt::new(1) - inv[cnt[i] as usize];
        cnt[i] -= 1;
    }
    println!("{ans}");
}
