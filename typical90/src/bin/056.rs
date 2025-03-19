use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        ab: [(usize, usize); n],
    }

    let mut dp = vec![vec![u8::MAX; s + 1]; n + 1];
    dp[0][0] = 0;
    for (i, &(ai, bi)) in ab.iter().enumerate() {
        if s >= ai {
            for j in 0..=s - ai {
                if dp[i][j] != u8::MAX {
                    dp[i + 1][j + ai] = b'A';
                }
            }
        }
        if s >= bi {
            for j in 0..=s - bi {
                if dp[i][j] != u8::MAX {
                    dp[i + 1][j + bi] = b'B';
                }
            }
        }
    }

    if dp[n][s] == u8::MAX {
        println!("Impossible");
    } else {
        let mut ans = Vec::with_capacity(n);

        let mut s = s;
        for i in (1..=n).rev() {
            let c = dp[i][s];
            ans.push(c);
            if c == b'A' {
                s -= ab[i - 1].0;
            } else {
                assert_eq!(c, b'B');
                s -= ab[i - 1].1;
            }
        }
        ans.reverse();
        println!("{}", std::str::from_utf8(&ans).unwrap());
    }
}
