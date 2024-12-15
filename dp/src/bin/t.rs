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
        n: usize,
        s: marker::Bytes,
    }

    let mut dp = vec![vec![MInt::new(0); n]; n];
    dp[0][0] = MInt::new(1);
    for i in 1..n {
        let c = s[i - 1];
        if c == b'>' {
            for j in 0..i {
                dp[i][j] = dp[i - 1][i - 1] - if j == 0 { MInt::new(0) } else { dp[i - 1][j - 1] };
            }
        } else {
            for j in 1..=i {
                dp[i][j] = dp[i - 1][j - 1];
            }
        }
        for j in 0..i {
            chsum!(dp[i][j + 1], dp[i][j]);
        }
    }
    println!("{}", dp[n - 1][n - 1]);
}
