use std::cmp::Reverse;

use proconio::input;
use segtree::{Segtree, operation::Operator};

#[derive(Default)]
struct Op;

impl Operator for Op {
    type Query = ((usize, usize), (usize, usize));
    const IDENT: Self::Query = ((0, 0), (0, 0));

    fn op(&self, a: &Self::Query, b: &Self::Query) -> Self::Query {
        let mut arr = [a.0, a.1, b.0, b.1];
        arr.sort_unstable_by_key(|x| Reverse(x.0));
        let mut f = arr[0];
        let mut i = 1;
        while i < 4 && f.0 == arr[i].0 {
            f.1 += arr[i].1;
            i += 1;
        }
        if i >= 4 {
            return (f, (0, 0));
        }
        let mut s = arr[i];
        i += 1;
        while i < 4 && s.0 == arr[i].0 {
            s.1 += arr[i].1;
            i += 1;
        }
        (f, s)
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        mut queries: [(u8, usize, usize); q],
    }
    let mut segtree = a.iter().map(|&ai| ((ai, 1), (0, 0))).collect::<Segtree<_ ,Op>>();

    for &(t, v1, v2) in &queries {
        if t == 1 {
            let p = v1;
            let x = v2;
            segtree.update(p - 1, ((x, 1), (0, 0)));
        } else if t == 2 {
            let l = v1;
            let r = v2;
            let ans = segtree.query(l - 1..r);
            // eprintln!("{ans:?}");
            println!("{}", ans.1.1);
        } else {
            unreachable!();
        }
    }
}
