use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        w: usize,
        n: usize,
        lrv: [(usize, usize, i64); n],
    }

    let mut dp = vec![i64::MIN; w + 1];
    dp[0] = 0;
    let mut dp_nxt = dp.clone();
    let mut set = BTreeSet::new();
    for &(li, ri, vi) in &lrv {
        dp_nxt.clone_from_slice(&dp);
        set.clear();
        for k in li..=ri {
            set.insert((dp[k - li], k - li));
            dp_nxt[k] = dp_nxt[k].max(set.last().unwrap().0 + vi);
        }
        for k in ri + 1..=w {
            set.remove(&(dp[k - ri - 1], k - ri - 1));
            set.insert((dp[k - li], k - li));
            dp_nxt[k] = dp_nxt[k].max(set.last().unwrap().0 + vi);
        }
        dp.clone_from_slice(&dp_nxt);
    }
    // eprintln!("{dp:?}");
    let ans = dp[w];
    if ans < 0 {
        println!("-1");
    } else {
        println!("{ans}");
    }
}
