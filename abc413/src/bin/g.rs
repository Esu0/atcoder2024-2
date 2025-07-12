use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        mut rc: [(usize, usize); k],
    }

    rc.iter_mut().for_each(|ij| {
        ij.0 -= 1;
        ij.1 -= 1;
    });
    rc.sort_unstable();
    let mut vis = vec![false; k];
    let mut stack = Vec::with_capacity(k);

    for j in 0..w {
        if let Ok(idx) = rc.binary_search(&(0, j)) {
            vis[idx] = true;
            stack.push(idx);
        }
    }
    for i in 1..h {
        if let Ok(idx) = rc.binary_search(&(i, w - 1)) {
            vis[idx] = true;
            stack.push(idx);
        }
    }

    let drc = [
        (0, 1),
        (1, 1),
        (1, 0),
        (1, usize::MAX),
        (0, usize::MAX),
        (usize::MAX, usize::MAX),
        (usize::MAX, 0),
        (usize::MAX, 1),
    ];
    // assert_eq!(std::collections::BTreeSet::from(drc).len(), 8);
    while let Some(idx) = stack.pop() {
        let (r, c) = rc[idx];
        for (dr, dc) in drc {
            let (nr, nc) = (r.wrapping_add(dr), c.wrapping_add(dc));
            if let Ok(idx2) = rc.binary_search(&(nr, nc)) {
                if !vis[idx2] {
                    vis[idx2] = true;
                    stack.push(idx2);
                }
            }
        }
    }

    for i in 0..h - 1 {
        if let Ok(idx) = rc.binary_search(&(i, 0)) {
            if vis[idx] {
                println!("No");
                return;
            }
        }
    }
    for j in 0..w {
        if let Ok(idx) = rc.binary_search(&(h - 1, j)) {
            if vis[idx] {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
