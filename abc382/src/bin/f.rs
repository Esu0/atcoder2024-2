use proconio::input;
use ac_library::{LazySegtree, MapMonoid, Monoid};

struct Min;

impl Monoid for Min {
    type S = usize;

    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        std::cmp::min(*a, *b)
    }

    fn identity() -> Self::S {
        usize::MAX
    }
}

struct MinUpdate;

impl MapMonoid for MinUpdate {
    type F = usize;
    type M = Min;

    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        if *f == usize::MAX {
            *g
        } else {
            *f
        }
    }

    fn identity_map() -> Self::F {
        usize::MAX
    }

    fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        if *f != usize::MAX {
            *f
        } else {
            *x
        }
    }
}

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        rcl: [(usize, usize, usize); n],
    }

    let mut blks = vec![vec![]; h];
    for (i, &(ri, _, _)) in rcl.iter().enumerate() {
        blks[ri - 1].push(i);
    }

    let mut ans = vec![usize::MAX; n];
    let base = (0..w).map(|_| h).collect::<Vec<_>>();
    let mut segtree = LazySegtree::<MinUpdate>::from(base);

    for r in (0..h).rev() {
        for &i in &blks[r] {
            let (_, ci, li) = rcl[i];
            let mn = segtree.prod(ci - 1..ci - 1 + li);
            ans[i] = mn;
            segtree.apply_range(ci - 1..ci - 1 + li, mn - 1);
        }
    }
    for &ansi in &ans {
        println!("{}", ansi);
    }
}
