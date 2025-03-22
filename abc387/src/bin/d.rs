use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        g: [proconio::marker::Bytes; h],
    }

    let mut s = (usize::MAX, usize::MAX);
    let mut t = s;
    for (i, row) in g.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == b'S' {
                s = (i, j);
            } else if cell == b'G' {
                t = (i, j);
            }
        }
    }

    let mut dist = vec![vec![[usize::MAX; 2]; w]; h];

    dist[s.0][s.1] = [0; 2];
    let mut queue = VecDeque::from([(s, 0), (s, 1)]);

    let dxy = [[(0, 1), (0, usize::MAX)], [(1, 0), (usize::MAX, 0)]];
    while let Some((u, dir)) = queue.pop_front() {
        let d = dist[u.0][u.1][dir];
        for (di, dj) in dxy[dir] {
            let ni = u.0.wrapping_add(di);
            let nj = u.1.wrapping_add(dj);
            if (0..h).contains(&ni) && (0..w).contains(&nj) && dist[ni][nj][1 - dir] == usize::MAX && g[ni][nj] != b'#' {
                dist[ni][nj][1 - dir] = d + 1;
                queue.push_back(((ni, nj), 1 - dir));
            }
        }
    }

    println!("{}", dist[t.0][t.1][0].min(dist[t.0][t.1][1]) as isize);
}
