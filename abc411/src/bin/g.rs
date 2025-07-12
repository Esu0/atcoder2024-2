use proconio::input;
type MInt = util::ModInt<998244353>;

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut mat = vec![vec![0usize; n]; n];
    for _ in 0..m {
        input! { u: usize, v: usize }
        mat[u - 1][v - 1] += 1;
        mat[v - 1][u - 1] += 1;
    }

    let mut dp = vec![vec![MInt::new(0); n]; 1 << (n - 1)];
    let mut popcount = vec![0u8; 1 << (n - 1)];
    for i in 0..n - 1 {
        for s in 0..1 << i {
            popcount[s | (1 << i)] = popcount[s] + 1;
        }
    }

    let mut idxss = vec![vec![]; n + 1];
    for s in 0..1 << (n - 1) {
        idxss[popcount[s] as usize].push(s);
    }
    idxss.iter_mut().for_each(|v| v.push(usize::MAX));

    let mut ans = MInt::new(0);
    for start in 0..n {
        dp.iter_mut().for_each(|dpi| dpi.fill(MInt::new(0)));
        dp[0][start] = MInt::new(1);
        for idxs in idxss[..start].iter() {
            let mut l = 0;
            while idxs[l] < (1 << start) {
                let s = idxs[l];
                for i in 0..=start {
                    if i != start && (s >> i) & 1 == 0 {
                        continue;
                    }
                    for j in 0..start {
                        if (s >> j) & 1 != 0 {
                            continue;
                        }
                        let tmp = dp[s | (1 << j)][j] + MInt::new(mat[i][j] as _) * dp[s][i];
                        dp[s | (1 << j)][j] = tmp;
                    }
                }
                l += 1;
            }
        }
        for s in 0..1 << start {
            if popcount[s] < 2 {
                continue;
            }
            for i in 0..start {
                ans += MInt::new(mat[i][start] as _) * dp[s][i];
            }
        }
    }
    for i in 0..n {
        for j in i + 1..n {
            let c = MInt::new(mat[i][j] as _);
            ans += c * (c - MInt::new(1));
        }
    }
    ans *= MInt::new(2).inv();
    println!("{ans}");
}
