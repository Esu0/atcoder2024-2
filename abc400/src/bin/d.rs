use std::cmp::Reverse;

use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [proconio::marker::Bytes; h],
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    }

    let mut queue = std::collections::BinaryHeap::new();
    queue.push((Reverse(0), a - 1, b - 1));
    let mut dist = vec![vec![usize::MAX; w]; h];
    let dij = [(0, 1), (0, 2), (0, usize::MAX), (0, usize::MAX - 1), (1, 0), (2, 0), (usize::MAX, 0), (usize::MAX - 1, 0)];
    while let Some((Reverse(d), i, j)) = queue.pop() {
        if dist[i][j] != usize::MAX {
            continue;
        }
        dist[i][j] = d;
        for (di, dj) in dij {
            let ni = i.wrapping_add(di);
            let nj = j.wrapping_add(dj);
            if ni < h && nj < w && dist[ni][nj] == usize::MAX {
                queue.push((Reverse(d + 1), ni, nj));
            }
        }

        for (di, dj) in [(0, 1), (0, usize::MAX), (1, 0), (usize::MAX, 0)] {
            let ni = i.wrapping_add(di);
            let nj = j.wrapping_add(dj);
            if ni < h && nj < w && s[ni][nj] == b'.' && dist[ni][nj] == usize::MAX {
                queue.push((Reverse(d), ni, nj));
            }
        }
    }
    println!("{}", dist[c - 1][d - 1]);
}
