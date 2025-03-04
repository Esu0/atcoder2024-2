use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }

    // let mut g = vec![vec![]; n];
    // for (i, &(ai, bi)) in ab.iter().enumerate() {
    //     g[ai - 1].push((bi - 1, i));
    //     g[bi - 1].push((ai - 1, i));
    // }

    let mut uf = union_find::UnionFind::new(vec![(); n]);
    let mut rem = vec![];
    for (i, &(ai, bi)) in ab.iter().enumerate() {
        if !uf.unite(ai - 1, bi - 1) {
            rem.push(i);
        }
    }
    let mut heads = std::collections::BTreeSet::new();
    for i in 0..n {
        heads.insert(uf.find_rc(i));
    }
    let mut ans = vec![];
    for _ in 0..heads.len() - 1 {
        let x = rem.pop().unwrap();
        let (ax, _) = ab[x];
        let x_head = uf.find_rc(ax - 1);
        assert!(heads.remove(&x_head));
        let y = heads.pop_first().unwrap();
        uf.unite(x_head, y);
        let new_head = uf.find_rc(y);
        heads.insert(new_head);
        ans.push((x, ax - 1, y));
    }
    println!("{}", ans.len());
    ans.iter().for_each(|&(x, y, z)| println!("{} {} {}", x + 1, y + 1, z + 1));
}
