use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }

    if n != m {
        println!("No");
        return;
    }

    let mut g = vec![vec![]; n];
    for &(ai, bi) in &ab {
        g[ai - 1].push(bi - 1);
        g[bi - 1].push(ai - 1);
    }

    if g.iter().any(|gi| gi.len() != 2) {
        println!("No");
        return;
    }

    let mut uf = union_find::UnionFind::new(vec![(); n]);
    for &(ai, bi) in &ab {
        uf.unite(ai - 1, bi - 1);
    }
    if uf.size(0) != n {
        println!("No");
        return;
    }
    println!("Yes");
}
