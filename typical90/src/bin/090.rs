use proconio::input;
type MInt = util::ModInt<998244353>;

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    if k * k > 10000000 {
        panic!()
    }
    if n * k > 100000000 {
        panic!();
    }
    let mut dp = vec![MInt::new(1); k + 2];
    let mut dp_nxt = vec![MInt::new(0); k + 2];
    let mut prev_m = 2;
    for m in (1..k).rev().map(|i| k / i + 1) {
        dp_nxt[..=m].fill(MInt::new(0));
        dp_nxt[0] = MInt::new(1);

        for i in 0..m {
            for j in 1..=prev_m {
                let ni = i + j;
                if ni <= m {
                    let tmp = dp[j] * dp_nxt[i];
                    dp_nxt[ni] += tmp;
                }
            }
        }
        prev_m = m;
        dp[..=m].clone_from_slice(&dp_nxt[..=m]);
    }

    for &dpi in &dp {
        eprint!("{} ", dpi);
    }
    eprintln!();

    let mut dp2 = vec![MInt::new(0); n + 2];
    dp2[0] = MInt::new(1);
    for i in 0..=n + 1 {
        for j in 1..=k + 1 {
            let ni = i + j;
            if ni <= n + 1 {
                let tmp = dp[j] * dp2[i];
                dp2[ni] += tmp;
            }
        }
    }
    println!("{}", dp2[n + 1]);
}
