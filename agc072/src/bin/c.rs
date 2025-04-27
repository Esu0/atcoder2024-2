use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let mut dp = vec![vec![0; n]; n];

    dp[0][0] = k - 1;
    for i in 0..n {
        for j in 0..n - i {
            let t = i + j + 2;
            let f = dp[i][j] / t;
            let rem = dp[i][j] % t;
            let d = f * (i + 1) + rem.min(i + 1);
            if i + 1 < n {
                dp[i + 1][j] += d;
            }
            if j + 1 < n {
                dp[i][j + 1] += dp[i][j] - d;
            }
        }
    }
    // dp.iter().for_each(|dpi| eprintln!("{dpi:?}"));

    let mut ans = Vec::with_capacity(2 * (n - 1));
    let (mut i, mut j) = (0, 0);
    while i + j != n - 1 {
        if dp[i][j + 1] < dp[i + 1][j] {
            ans.push(b'R');
            j += 1;
        } else {
            ans.push(b'D');
            i += 1;
        }
    }
    let tmp = ans
        .iter()
        .rev()
        .map(|&x| if x == b'R' { b'D' } else { b'R' })
        .collect::<Vec<_>>();
    ans.extend(tmp);
    println!("{}", std::str::from_utf8(&ans).unwrap());
}
