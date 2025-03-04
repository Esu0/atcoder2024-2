use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut clr: [(u64, usize, usize); m],
    }

    clr.sort_unstable_by_key(|&(ci, _, _)| ci);

    for (_, li, _) in &mut clr {
        *li -= 1;
    }

    let mut uf = union_find::UnionFind::new(vec![(); n + 1]);
    let mut count = n + 1;
    let mut ans = 0;
    for &(ci, li, ri) in &clr {
        if uf.unite(li, ri) {
            count -= 1;
            ans += ci;
        }
        if count == 1 {
            break;
        }
    }
    if count != 1 {
        println!("-1");
    } else {
        println!("{ans}");
    }
}
