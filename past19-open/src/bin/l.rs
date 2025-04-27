use proconio::input;
use segtree::{Segtree, operation};

fn main() {
    input! {
        n: usize,
        b: [u32; n],
    }

    let mut b_sorted = b.clone();
    b_sorted.sort_unstable();
    let mut b = b.iter().map(|&bi| b_sorted.partition_point(|&x| x < bi)).collect::<Vec<_>>();
    
}
