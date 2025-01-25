use std::cmp::Reverse;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        ab: [(u64, u64); n],
    }

    let mut arr = ab
        .iter()
        .flat_map(|&(ai, bi)| [ai - bi, bi])
        .collect::<Vec<_>>();
    arr.sort_unstable_by_key(|x| Reverse(*x));
    let ans = arr[..k].iter().sum::<u64>();
    println!("{ans}");
}
