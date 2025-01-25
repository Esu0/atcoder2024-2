use proconio::{input, marker};

macro_rules! chmin {
    ($d:expr, $s:expr) => {
        {
            let tmp = $d;
            $d = tmp.min($s);
        }
    };
}

fn main() {
    input! {
        k: usize,
        s: marker::Bytes,
        t: marker::Bytes,
    }
    let n = s.len();
    let m = t.len();
    let mut dp = vec![vec![usize::MAX; m + 1]; n + 1];
    dp[0][0] = 0;
    for i in 0..n {
        for j in 0..m {
            chmin!(dp[i + 1][j], dp[i][j] + 1);
            chmin!(dp[i][j + 1], dp[i][j] + 1);
            chmin!(dp[i + 1][j + 1], dp[i][j] + if s[i] == t[j] { 0 } else { 1 });
        }
    }
    dp.iter().for_each(|dpi| eprintln!("{dpi:?}"));
}
