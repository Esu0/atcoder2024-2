use proconio::input;
use ac_library::{LazySegtree, MapMonoid, Monoid};
use segtree::{Segtree, operation};
type MInt = util::ModInt<998244353>;

struct Sum;

impl Monoid for Sum {
    type S = MInt;
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        *a + *b
    }

    fn identity() -> Self::S {
        MInt::new(0)
    }
}

struct SumProd;

impl MapMonoid for SumProd {
    type F = MInt;
    type M = Sum;

    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        *f * *g
    }

    fn identity_map() -> Self::F {
        MInt::new(1)
    }

    fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        *x * *f
    }
}

fn main() {
    input! {
        q: usize,
        k: u32,
        queries: [(char, u32); q],
    }
    let mut num_sorted = queries.iter().map(|&(_, x)| x).collect::<Vec<_>>();
    num_sorted.sort_unstable();
    num_sorted.dedup();
    // let mut count = HashMap::new();
    // for &n in &num_sorted {
    //     count.insert(n, 0);
    // }

    let k = MInt::new(k as _);
    let kinv = k.inv();
    let mut segtree = LazySegtree::<SumProd>::new(num_sorted.len());
    let mut segtree_ind = Segtree::<_, operation::Add<_>>::from_iter((0..num_sorted.len()).map(|_| 0usize));
    for &(c, x) in &queries {
        let ind = num_sorted.partition_point(|&y| y < x);
        let x = MInt::new(x as _);
        let p = segtree_ind.query(..=ind);
        if c == '+' {
            *segtree_ind.get_mut(ind) += 1;
            let prev = segtree.get(ind);
            segtree.set(ind, prev + x * k.pow(p as u32));
            segtree.apply_range(ind + 1.., k);
        } else {
            *segtree_ind.get_mut(ind) -= 1;
            let prev = segtree.get(ind);
            segtree.set(ind, prev - x * k.pow((p - 1) as u32));
            segtree.apply_range(ind + 1.., kinv);
        }
        println!("{}", segtree.all_prod());
    }
}
