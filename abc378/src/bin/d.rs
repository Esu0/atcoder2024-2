use proconio::input;

fn dfs(grid: &mut [Vec<u8>], i: usize, j: usize, k: usize, cnt: &mut usize) {
    if k == 0 {
        *cnt += 1;
        return;
    }
    let h = grid.len();
    let w = grid[0].len();
    let dij = [(usize::MAX, 0), (1, 0), (0, usize::MAX), (0, 1)];

    for (di, dj) in dij {
        let ni = i.wrapping_add(di);
        let nj = j.wrapping_add(dj);
        if (0..h).contains(&ni) && (0..w).contains(&nj) && grid[ni][nj] == b'.' {
            grid[ni][nj] = 0;
            dfs(grid, ni, nj, k - 1, cnt);
            grid[ni][nj] = b'.';
        }
    }
}

fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        mut s: [proconio::marker::Bytes; h],
    }

    let mut cnt = 0;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == b'.' {
                s[i][j] = 0;
                dfs(&mut s, i, j, k, &mut cnt);
                s[i][j] = b'.';
            }
        }
    }
    println!("{cnt}");
}
