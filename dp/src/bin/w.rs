use proconio::input;
use ac_library::{LazySegtree, MapMonoid, Monoid};

fn main() {
    input! {
        n: usize,
        m: usize,
        mut lra: [(usize, usize, i64); m],
    }

    lra.sort_unstable_by_key(|lra| lra.1);
    // let mut map = vec![0; n + 1];
    // for &(li, ri, ai) in &lra {
    //     map[li - 1] += ai;
    //     map[ri] -= ai;
    // }
    // for i in 0..n {
    //     map[i + 1] += map[i];
    // }

    struct Op;
    struct MaxOp;
    impl Monoid for MaxOp {
        type S = i64;
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            (*a).max(*b)
        }

        fn identity() -> Self::S {
            i64::MIN
        }
    }
    impl MapMonoid for Op {
        type M = MaxOp;
        type F = i64;
        fn composition(f: &Self::F, g: &Self::F) -> Self::F {
            *f + *g
        }

        fn identity_map() -> Self::F {
            0
        }

        fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
            if *x != i64::MIN { *x + *f } else { i64::MIN }
        }
    }
    let mut segtree = LazySegtree::<Op>::new(n + 1);
    // let mut dp = vec![0; n + 1];
    let mut j = 0;
    segtree.set(0, 0);
    for i in 0..n {
        let tmp = segtree.prod(..i).max(0);
        segtree.set(i, tmp);
        while j < m && lra[j].1 <= i + 1 {
            let (lj, rj, aj) = lra[j];
            segtree.apply_range(lj - 1..rj, aj);
            j += 1;
        }
    }
    println!("{}", segtree.prod(..n).max(0));
}
