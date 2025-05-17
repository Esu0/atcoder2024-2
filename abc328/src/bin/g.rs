use std::collections::HashMap;

use proconio::input;

/// O(N^2 * 2^N)
fn _solve1(c: u64, a: &[u64], b: &[u64]) -> u64 {
    let n = a.len();
    assert_eq!(b.len(), n);
    let mut dp = vec![vec![u64::MAX; n]; 1 << n];
    let mut popcount = vec![usize::MAX; 1 << n];
    popcount[0] = 0;
    for i in 0..n {
        dp[1 << i][i] = a[i].abs_diff(b[0]);
        popcount[1 << i] = 1;
    }

    for s in 1..1 << n {
        let cnt = popcount[s];
        for i in 0..n {
            if (s >> i) & 1 == 0 {
                popcount[s | (1 << i)] = cnt + 1;
                continue;
            }
            if cnt == 1 {
                continue;
            }
            let t = s & !(1 << i);
            for j in 0..n {
                if (t >> j) & 1 == 0 {
                    continue;
                }
                if j + 1 == i {
                    dp[s][i] = dp[s][i].min(dp[t][j] + b[cnt - 1].abs_diff(a[i]));
                } else {
                    dp[s][i] = dp[s][i].min(dp[t][j] + c + b[cnt - 1].abs_diff(a[i]));
                }
            }
        }
    }
    dp[(1 << n) - 1].iter().copied().min().unwrap()
}

fn solve2(c: u64, a: &[u64], b: &[u64]) -> u64 {
    let n = a.len();
    assert_eq!(b.len(), n);
    let mut dp = vec![vec![u64::MAX; n]; 1 << n];
    // let mut dpi = HashMap::new();
    let mut dp2 = vec![u64::MAX; 1 << n];
    let mut popcount = vec![usize::MAX; 1 << n];
    popcount[0] = 0;
    for i in 0..n {
        let x = a[i].abs_diff(b[0]);
        dp[1 << i][i] = x;
        // dpi.insert((1u32 << i, i as u8), x);
        dp2[1 << i] = x;
        popcount[1 << i] = 1;
    }

    // let mut indice = vec![vec![]; n + 1];
    // for s in 0..1 << n {
    //     let cnt = popcount[s];
    //     indice[cnt].push(s);
    //     for i in 0..n {
    //         if (s >> i) & 1 == 0 {
    //             popcount[s | (1 << i)] = cnt + 1;
    //         }
    //     }
    // }

    // let mut dpi_nxt = HashMap::new();
    // for cnt in 2..=n {
    //     dpi_nxt.clear();
    //     for &s in &indice[cnt] {
    //         for i in 0..n {
    //             if (s >> i) & 1 == 0 {
    //                 continue;
    //             }
    //             let t = s & !(1 << i);
    //             let x = b[cnt - 1].abs_diff(a[i]);
    //             let mut new = dp2[t] + c + x;
    //             if i > 0 && (t >> (i - 1)) & 1 != 0 {
    //                 new = new.min(dpi[&(t as u32, i as u8 - 1)] + x);
    //             }
    //             dpi_nxt.insert((s as u32, i as u8), new);
    //             dp2[s] = dp2[s].min(new);
    //         }
    //     }
    //     dpi.clone_from(&dpi_nxt);
    // }
    for s in 1..1 << n {
        let cnt = popcount[s];
        for i in 0..n {
            if (s >> i) & 1 == 0 {
                popcount[s | (1 << i)] = cnt + 1;
                continue;
            }
            if cnt == 1 {
                continue;
            }
            let t = s & !(1 << i);
            let x = b[cnt - 1].abs_diff(a[i]);
            let mut new = dp2[t] + c + x;
            if i > 0 && (t >> (i - 1)) & 1 != 0 {
                new = new.min(dp[t][i - 1] + x);
            }
            dp[s][i] = new;
            dp2[s] = dp2[s].min(new);
        }
    }
    dp2[(1 << n) - 1]
}

fn main() {
    input! {
        n: usize,
        c: u64,
        a: [u64; n],
        b: [u64; n],
    }

    // let expected = solve1(c, &a, &b);
    let ans = solve2(c, &a, &b);
    // assert_eq!(expected, ans);
    println!("{ans}");
}
