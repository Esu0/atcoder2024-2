use proconio::{input, marker};
use util::ModInt;
type MInt = ModInt<1000000007>;

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
        k: marker::Bytes,
        d: usize,
    }
    let n = k.len();
    let mut dp = vec![vec![MInt::new(0); d]; n + 1];
    dp[0][0] = MInt::new(1);
    for i in 0..n {
        for j in 0..d {
            for k in 0..10 {
                chsum!(dp[i + 1][(j + k) % d], dp[i][j]);
            }
        }
    }
    // eprintln!("{dp:?}");
    let mut sum = k.iter().map(|&d| (d - b'0') as usize).collect::<Vec<_>>();
    for i in 1..n {
        sum[i] += sum[i - 1];
    }
    let mut ans = MInt::new(0);
    for (i, &ki) in k.iter().enumerate().rev() {
        let ki = ki - b'0';
        let offset = (sum[i] - ki as usize) % d;
        for l in 0..ki {
            let offset2 = (offset + l as usize) % d;
            // eprintln!("{ans}");
            ans += dp[n - i - 1][(d - offset2) % d];
        }
    }
    ans -= MInt::new(1);
    if *sum.last().unwrap() % d == 0 {
        ans += MInt::new(1);
    }
    println!("{ans}");
}
