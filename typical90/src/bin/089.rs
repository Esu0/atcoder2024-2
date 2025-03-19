use proconio::input;
use segtree::{operation, Segtree};
type MInt = util::ModInt<1000000007>;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [u32; n],
    }

    let mut a_sorted = a.clone();
    a_sorted.sort_unstable();
    let na = a.iter().map(|&ai| {
        a_sorted.partition_point(|&x| x < ai)
    }).collect::<Vec<_>>();

    let mut rs = vec![usize::MAX; n];
    let mut segtree = (0..n).map(|_| 0usize).collect::<Segtree<_, operation::Add<_>>>();

    let mut j = 0;
    let mut rev = 0;
    *segtree.get_mut(na[0]) += 1;
    for (&ai, dst) in na.iter().zip(&mut rs) {
        while j < n - 1 && rev <= k {
            j += 1;
            rev += segtree.query(na[j] + 1..);
            *segtree.get_mut(na[j]) += 1;
        }
        if j < n && rev <= k {
            j += 1;
        }
        *dst = j;
        *segtree.get_mut(ai) -= 1;
        rev -= segtree.query(..ai);
    }

    // eprintln!("{rs:?}");
    #[derive(Default)]
    struct Op;
    impl operation::Operator for Op {
        type Query = MInt;
        const IDENT: Self::Query = MInt::new(0);
        fn op(&self, a: &Self::Query, b: &Self::Query) -> Self::Query {
            *a + *b
        }

        fn op_assign_left(&self, a: &mut Self::Query, b: &Self::Query) {
            *a += *b;
        }

        fn op_assign_right(&self, a: &Self::Query, b: &mut Self::Query) {
            *b += *a;
        }
    }

    let mut dp = (0..=n).map(|_| MInt::new(0)).collect::<Segtree<_, Op>>();

    dp.update(n, MInt::new(1));
    for (i, &ri) in rs.iter().enumerate().rev() {
        let s = dp.query(i + 1..=ri);
        dp.update(i, s);
    }
    println!("{}", dp[0]);
}
