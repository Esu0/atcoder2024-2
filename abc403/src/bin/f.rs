use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut dp = vec![[Vec::new(), Vec::new()]; 2001];
    dp[1][0].push(b'1');
    dp[1][1].push(b'1');
    dp[11][0].extend(*b"11");
    dp[11][1].extend(*b"11");
    dp[111][0].extend(*b"111");
    dp[111][1].extend(*b"111");
    dp[1111][0].extend(*b"1111");
    dp[1111][1].extend(*b"1111");

    for i in 2..=n {
        for j in 1..i {
            // 足し算
            if dp[i][0].is_empty() || dp[i][0].len() > dp[j][0].len() + dp[i - j][0].len() + 1 {
                let mut tmp = dp[j][0].clone();
                tmp.push(b'+');
                tmp.extend(dp[i - j][0].iter().copied());
                dp[i][0] = tmp;
            }

            // 掛け算
            if j != 1 && i % j == 0 && (dp[i][1].is_empty() || dp[i][1].len() > dp[j][1].len() + dp[i / j][1].len() + 1) {
                let mut tmp = dp[j][1].clone();
                tmp.push(b'*');
                tmp.extend(dp[i / j][1].iter().copied());
                dp[i][1] = tmp;
            }

            // 括弧
            if dp[i][1].is_empty() || dp[i][1].len() > dp[i][0].len() + 2 {
                let mut tmp = Vec::with_capacity(dp[i][0].len() + 2);
                tmp.push(b'(');
                tmp.extend(dp[i][0].iter().copied());
                tmp.push(b')');
                dp[i][1] = tmp;
            }
            if dp[i][0].len() > dp[i][1].len() {
                let tmp = dp[i][1].clone();
                dp[i][0] = tmp;
            }
        }
    }
    println!("{}", std::str::from_utf8(&dp[n][0]).unwrap());
}
