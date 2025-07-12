use proconio::input;

fn main() {
    input! {
        n: usize,
        c: u64,
        a: [u64; n],
        b: [u64; n],
    }

    let mut dp = vec![u64::MAX; 1 << n];
    let popcount = (0usize..1 << n).map(|x| x.count_ones() as u8).collect::<Vec<_>>();
    let mut idxs = (0..(1 << n) - 1).collect::<Vec<_>>();
    idxs.sort_unstable_by_key(|&x| popcount[x]);

    dp[0] = 0;
    for &s in &idxs {
        let pop = popcount[s];
        for l in 0..n {
            let mut sum = c;
            let mut t = s;
            for r in l..n {
                if (s >> r) & 1 != 0 {
                    break;
                }
                t |= 1 << r;
                sum += a[r].abs_diff(b[pop as usize + r - l]);
                dp[t] = dp[t].min(dp[s] + sum);
            }
        }
    }
    println!("{}", dp[(1 << n) - 1] - c);
}
