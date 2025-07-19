use proconio::input;
use union_find::UnionFind;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut uvw: [(usize, usize, u32); m],
    }

    for (ui, vi, _) in &mut uvw {
        *ui -= 1;
        *vi -= 1;
    }

    let mut ans = 0u32;
    let mut need = 0;
    for i in (0..30).rev() {
        let mut uf = UnionFind::new(vec![(); n]);
        need |= 1 << i;
        for &(ui, vi, wi) in &uvw {
            if wi & need == 0 {
                uf.unite(ui, vi);
            }
        }
        if uf.find_rc(0) != uf.find_rc(n - 1) {
            ans |= 1 << i;
            need ^= 1 << i;
        }
    }
    println!("{ans}");
}
