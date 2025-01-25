use proconio::input;
use segtree::{Segtree, operation};

fn main() {
    input! {
        n: usize,
        a: [u64; n],
        q: usize,
        lr: [(usize, usize); q],
    }

    #[derive(Default)]
    struct Op;

    impl operation::Operator for Op {
        type Query = (usize, usize);
        const IDENT: Self::Query = (0, 0);

        fn op(&self, a: &Self::Query, b: &Self::Query) -> Self::Query {
            (a.0.max(b.0), a.1 + b.1)
        }
    }

    let segtree = a.iter().enumerate().map(|(i, &ai)| {
        let tmp = a[i..].partition_point(|&x| x < 2 * ai);
        assert_ne!(tmp, 0);
        (tmp, 1)
    }).collect::<Segtree<(usize, usize), Op>>();


    for &(li, ri) in &lr {
        let ans = segtree.upper_bound(li - 1, |&(x, d)| {
            d + d.max(x) <= ri - li + 1
        }) - (li - 1);
        println!("{ans}");
    }
}
