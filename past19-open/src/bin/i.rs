use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut abd: [(usize, usize, u32); m],
    }
    abd.sort_unstable_by_key(|abdi| std::cmp::Reverse(abdi.2));

    let mut uf = union_find::UnionFind::new(vec![(); n]);
    let mut cnt = n;
    for &(ai, bi, di) in &abd {
        if uf.unite(ai - 1, bi - 1) {
            cnt -= 1;
        }

        if cnt == 1 {
            println!("{di}");
            return;
        }
    }
}
