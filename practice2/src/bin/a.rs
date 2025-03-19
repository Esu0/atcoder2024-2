use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut uf = union_find::UnionFind::new(vec![(); n]);

    for _ in 0..q {
        input! { t: u8, u: usize, v: usize }

        if t == 0 {
            uf.unite(u, v);
        } else if uf.find_rc(u) == uf.find_rc(v) {
            println!("1");
        } else {
            println!("0");
        }
    }
}
