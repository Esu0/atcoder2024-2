use std::collections::BTreeSet;

use proconio::input_interactive;

fn main() {
    input_interactive! {
        n: usize,
        uv: [(usize, usize); n - 1],
    }

    let mut g = vec![vec![]; n];
    for &(ui, vi) in &uv {
        g[ui - 1].push(vi - 1);
        g[vi - 1].push(ui - 1);
    }

    let mut col = vec![usize::MAX; n];
    let mut stack = Vec::with_capacity(n);
    stack.push(0);
    col[0] = 0;
    while let Some(u) = stack.pop() {
        let color = col[u];
        for &v in &g[u] {
            if col[v] == usize::MAX {
                col[v] = 1 - color;
                stack.push(v);
            }
        }
    }

    let g0 = col.iter().enumerate().filter(|&(_, &c)| c == 0).map(|(i, _)| i).collect::<Vec<_>>();
    let g1 = col.iter().enumerate().filter(|&(_, &c)| c == 1).map(|(i, _)| i).collect::<Vec<_>>();
    eprintln!("{g0:?} {g1:?}");
    assert_eq!(g0.len() + g1.len(), n);
    // let edge_mx = g0.len() * g1.len();
    let mut edge_set = g0.iter().flat_map(|&i| g1.iter().map(move |&j| if i > j { (j, i) } else { (i, j) })).collect::<BTreeSet<_>>();
    for &(ui, vi) in &uv {
        assert!(edge_set.remove(&(ui - 1, vi - 1)));
    }
    let rem_edge = edge_set.len();

    if rem_edge % 2 == 1 {
        println!("First");
        let (i, j) = edge_set.pop_first().unwrap();
        println!("{} {}", i + 1, j + 1);
    } else {
        println!("Second");
    }

    loop {
        input_interactive! { i: isize, j: isize }
        if i == -1 {
            return;
        }
        let i = (i - 1) as usize;
        let j = (j - 1) as usize;
        assert!(edge_set.remove(&(i, j)));
        let (i, j) = edge_set.pop_first().unwrap();
        println!("{} {}", i + 1, j + 1);
    }
}
