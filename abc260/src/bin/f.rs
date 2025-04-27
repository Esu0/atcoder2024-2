use proconio::input;

fn main() {
    input! {
        s: usize,
        t: usize,
        m: usize,
        uv: [(usize, usize); m],
    }

    let mut g = vec![vec![]; s];
    for &(ui, vi) in &uv {
        g[ui - 1].push(vi - s - 1);
    }

    g.iter_mut().for_each(|gi| gi.sort_unstable());

    let mut set = vec![vec![usize::MAX; t]; t];

    for (i, gi) in g.iter().enumerate() {
        for (k, &u) in gi.iter().enumerate() {
            for &v in &gi[k + 1..] {
                assert!(u < v);
                if set[u][v] == usize::MAX {
                    set[u][v] = i;
                } else {
                    println!("{} {} {} {}", i + 1, set[u][v] + 1, u + s + 1, v + s + 1);
                    return;
                }
            }
        }
    }
    println!("-1");
}
