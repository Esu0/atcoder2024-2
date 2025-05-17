use proconio::input;

fn dfs(u: usize, g: &[Vec<usize>], map: &mut [usize], last: &mut [usize], id: &mut usize) {
    map[u] = *id;
    let mut l = *id;
    *id += 1;
    for &v in &g[u] {
        if map[v] == usize::MAX {
            dfs(v, g, map, last, id);
            l = last[map[v]];
        }
    }
    last[map[u]] = l;
}

fn main() {
    input! {
        n: usize,
        uv: [(usize, usize); n - 1],
        q: usize,
    }

    let mut g = vec![vec![]; n];
    for &(ui, vi) in &uv {
        g[ui - 1].push(vi - 1);
        g[vi - 1].push(ui - 1);
    }

    let mut map = vec![usize::MAX; n];
    let mut last = vec![usize::MAX; n];
    let mut id = 0;
    dfs(0, &g, &mut map, &mut last, &mut id);
    use segtree::{Segtree, operation};
    let mut segtree = (0..n).map(|_| 1u64).collect::<Segtree<_, operation::Add<_>>>();
    let mut all_sum = n as u64;

    use std::fmt::Write;
    let mut out_s = String::new();
    for _ in 0..q {
        input! { t: u8 }
        if t == 1 {
            input! { x: usize, w: u64 }
            let x = x - 1;
            *segtree.get_mut(map[x]) += w;
            all_sum += w;
        } else {
            input! { y: usize }
            let y = y - 1;
            let (uy, vy) = uv[y];
            let id = map[uy - 1].max(map[vy - 1]);
            let s = segtree.query(id..=last[id]);
            let ans = (all_sum - s).abs_diff(s);
            writeln!(out_s, "{ans}").unwrap();
        }
    }
    print!("{out_s}");
}
