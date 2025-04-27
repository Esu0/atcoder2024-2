use proconio::input;
type MInt = util::ModInt<998244353>;

macro_rules! chsum {
    ($d:expr, $s:expr) => {
        {
            let tmp = $d + $s;
            $d = tmp;
        }
    };
}

fn main() {
    input! {
        n: usize,
        x: usize,
        t0: usize,
        t: [usize; n - 1],
    }

    let mut dp = vec![MInt::new(0); x + 1];
    let ninv = MInt::new(n as _).inv();
    // dp[x] = ninv;
    for i in (0..=x).rev() {
        if i + t0 <= x {
            chsum!(dp[i], dp[i + t0] * ninv);
        } else {
            chsum!(dp[i], ninv);
        }
        for &ti in &t {
            if i + ti <= x {
                chsum!(dp[i], dp[i + ti] * ninv);
            }
        }
    }
    println!("{}", dp[0]);
}
