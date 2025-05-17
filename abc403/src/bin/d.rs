use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        a: [usize; n],
    }

    if d == 0 {
        let s = a.iter().copied().collect::<HashSet<_>>();
        println!("{}", n - s.len());
        return;
    }
    let mut dp = vec![[0; 2]; d];
    // for dpi in &mut dp {
    //     // 現在位置を採用できる場合
    //     dpi[0] = 0;
    // }

    let mut count = vec![0usize; 1_000_001];
    for &ai in &a {
        count[ai] += 1;
    }

    for i in 0..1_000_001 {
        let dpi = dp[i % d];
        dp[i % d][1] = dpi[0];
        dp[i % d][0] = dpi[0].min(dpi[1]) + count[i];
    }
    let ans = dp.iter().map(|&[a, b]| a.min(b)).sum::<usize>();
    println!("{ans}");
}
