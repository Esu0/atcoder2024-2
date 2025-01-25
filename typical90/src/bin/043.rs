use std::collections::VecDeque;

use proconio::{input, marker};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    Right,
    Up,
    Left,
    Down,
}

use Direction::*;

impl Direction {
    fn to_ind(self) -> usize {
        match self {
            Right => 0,
            Up => 1,
            Left => 2,
            Down => 3,
        }
    }
    fn dij(self) -> (usize, usize) {
        match self {
            Right => (0, 1),
            Up => (usize::MAX, 0),
            Left => (0, usize::MAX),
            Down => (1, 0),
        }
    }
}

fn main() {
    input! {
        h: usize,
        w: usize,
        rs: usize,
        cs: usize,
        rt: usize,
        ct: usize,
        s: [marker::Bytes; h],
    }

    let rs = rs - 1;
    let cs = cs - 1;
    let rt = rt - 1;
    let ct = ct - 1;
    let dirs = [Right, Up, Left, Down];
    let mut queue = dirs
        .iter()
        .map(|&d| (rs, cs, d, 0))
        .collect::<VecDeque<_>>();
    let mut dp = vec![vec![[u32::MAX; 4]; w]; h];
    dp[rs][cs].fill(0);
    while let Some((mut i, mut j, dir, dep)) = queue.pop_back() {
        let (di, dj) = dir.dij();
        let mut ni = i.wrapping_add(di);
        let mut nj = j.wrapping_add(dj);
        while (0..h).contains(&ni) && (0..w).contains(&nj) {
            if s[ni][nj] == b'#' {
                break;
            }
            if dp[ni][nj][dir.to_ind()] != u32::MAX {
                break;
            }
            dp[ni][nj][dir.to_ind()] = dep;
            i = ni;
            j = nj;
            for d in dirs {
                if d == dir {
                    continue;
                }
                let (di, dj) = d.dij();
                let ni = i.wrapping_add(di);
                let nj = j.wrapping_add(dj);
                // eprintln!("{i} {j}");
                if (0..h).contains(&ni)
                    && (0..w).contains(&nj)
                    && s[ni][nj] != b'#'
                    && dp[ni][nj][d.to_ind()] == u32::MAX
                {
                    queue.push_front((i, j, d, dep + 1));
                }
            }
            ni = i.wrapping_add(di);
            nj = j.wrapping_add(dj);
        }
    }
    // eprintln!("{dp:?}");
    println!("{}", dp[rt][ct].iter().copied().min().unwrap());
}
