use proconio::input;
use segtree::{Segtree, operation};

fn main() {
    input! {
        n: usize,
        lr: [(usize, usize); n],
        q: usize,
    }

    let mut segtree = (0..=500001).map(|i| if i == 0 {0usize} else {1usize}).collect::<Segtree<_, operation::Add<_>>>();

    for &(li, ri) in &lr {
        let l = segtree.upper_bound(0, |&x| x < li);
        let r = segtree.upper_bound(0, |&x| x <= ri);
        *segtree.get_mut(l) += 1;
        *segtree.get_mut(r) -= 1;
    }

    // eprintln!("{:?}", &segtree[..10]);
    for _ in 0..q {
        input! { x: usize }

        println!("{}", segtree.query(..=x));
    }

}
