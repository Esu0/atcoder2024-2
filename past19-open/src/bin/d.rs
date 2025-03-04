use std::cmp::Reverse;

use proconio::input;

fn main() {
    input! {
        n: usize,
        t: i64,
        mut ab: [(i64, i64); n],
    }

    let mut iab = ab.iter().copied().enumerate().collect::<Vec<_>>();

    iab.sort_unstable_by_key(|iab| (Reverse(iab.1.0), iab.1.1, iab.0));
    let (a, b) = iab[0].1;
    for &(ai, bi) in &ab {
        println!("{}", t * (a - ai) + (bi - b));
    }
}
