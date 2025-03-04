use std::cmp::Reverse;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut ab: [(usize, usize); m],
    }

    let mut dp = Vec::<usize>::with_capacity(n);

    ab.sort_unstable_by_key(|&(ai, bi)| (ai, Reverse(bi)));

    for &(_, bi) in &ab {
        let l = dp.partition_point(|&x| x < bi);
        if l == dp.len() {
            dp.push(bi);
        } else {
            dp[l] = bi;
        }
    }
    println!("{}", dp.len());
}
