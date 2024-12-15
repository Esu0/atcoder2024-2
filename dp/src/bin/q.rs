use proconio::input;
use segtree::{operation, Segtree};

fn main() {
    input! {
        n: usize,
        h: [usize; n],
        a: [u64; n],
    }

    let mut dp = (0..n).map(|_| 0).collect::<Segtree<u64, operation::Max<_>>>();
    let mut pos = vec![usize::MAX; n];
    for (i, &hi) in h.iter().enumerate() {
        pos[hi - 1] = i;
    }

    for hi in pos {
        let nxt = dp.query(0..hi) + a[hi];
        dp.update(hi, nxt);
    }
    println!("{}", dp.query(0..n));
}
