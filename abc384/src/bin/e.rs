use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        x: u64,
        p: usize,
        q: usize,
        s: [[u64; w]; h],
    }
    let p = p - 1;
    let q = q - 1;
    let mut queue = BinaryHeap::new();
    let dij = [(0, 1), (0, usize::MAX), (1, 0), (usize::MAX, 0)];
    let mut visited = vec![vec![false; w]; h];
    visited[p][q] = true;
    for (di, dj) in dij {
        let ni = p.wrapping_add(di);
        let nj = q.wrapping_add(dj);
        if (0..h).contains(&ni) && (0..w).contains(&nj) {
            queue.push((Reverse(s[ni][nj]), ni, nj));
            visited[ni][nj] = true;
        }
    }

    let mut ans = s[p][q];
    while let Some((Reverse(sij), i, j)) = queue.pop() {
        if (ans + x - 1) / x > sij {
            ans += sij;
            for (di, dj) in dij {
                let ni = i.wrapping_add(di);
                let nj = j.wrapping_add(dj);
                if (0..h).contains(&ni) && (0..w).contains(&nj) && !visited[ni][nj] {
                    queue.push((Reverse(s[ni][nj]), ni, nj));
                    visited[ni][nj] = true;
                }
            }
        } else {
            break;
        }
    }
    println!("{ans}");
}
