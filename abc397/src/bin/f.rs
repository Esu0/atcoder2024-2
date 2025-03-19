use proconio::input;
use ac_library::{LazySegtree, MapMonoid, Monoid};

struct Max;
struct MaxAdd;

impl Monoid for Max {
    type S = usize;
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        std::cmp::max(*a, *b)
    }

    fn identity() -> Self::S {
        0
    }
}

impl MapMonoid for MaxAdd {
    type F = isize;
    type M = Max;

    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        *f + *g
    }

    fn identity_map() -> Self::F {
        0
    }

    fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        x.checked_add_signed(*f).unwrap()
    }
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut s1 = vec![0; n + 1];
    let mut s2 = vec![0; n + 1];
    let mut l1 = 0usize;
    let mut l2 = 0usize;
    for &ai in &a {
        if s2[ai] == 0 {
            l2 += 1;
        }
        s2[ai] += 1;
    }

    let mut poss = vec![vec![]; n];
    for i in (0..n).rev() {
        poss[a[i] - 1].push(i);
    }

    let mut b = vec![usize::MAX; n];
    for (i, &ai) in a.iter().enumerate() {
        b[i] = l1 + l2;
        if s1[ai] == 0 {
            l1 += 1;
        }
        s1[ai] += 1;
        s2[ai] -= 1;
        if s2[ai] == 0 {
            l2 -= 1;
        }
    }

    // eprintln!("{b:?}");
    // eprintln!("{:?}", poss);
    let mut segtree = LazySegtree::<MaxAdd>::from(b);

    let mut ans = 0;
    let mut s = vec![false; n];
    let mut cnt_s = 0;
    for i in 0..n {
        ans = ans.max(cnt_s + segtree.prod(i..n));
        let ai = a[i] - 1;
        poss[ai].pop();
        if let Some(&j) = poss[ai].last() {
            segtree.apply_range(i..=j, -1);
        } else {
            segtree.apply_range(i..n, -1);
        }
        // eprintln!("{poss:?}");
        // eprintln!("{:?}", segtree);
        if !s[ai] {
            cnt_s += 1;
            s[ai] = true;
        }
    }
    println!("{ans}");
}
