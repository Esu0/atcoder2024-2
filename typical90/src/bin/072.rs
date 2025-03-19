use proconio::input;

fn solve(grid: &mut [Vec<u8>], s: usize, t: usize, cnt: usize, ans: &mut usize) {
    let h = grid.len();
    let w = grid[0].len();
    let sr = s / w;
    let sc = s % w;
    let tr = t / w;
    let tc = t % w;
    for (dr, dc) in [(0, 1), (0, usize::MAX), (1, 0), (usize::MAX, 0)] {
        let nr = tr.wrapping_add(dr);
        let nc = tc.wrapping_add(dc);
        if nr == sr && nc == sc {
            *ans = std::cmp::max(*ans, cnt);
        }
        if (0..h).contains(&nr) && (0..w).contains(&nc) && grid[nr][nc] == b'.' {
            grid[nr][nc] = 0;
            solve(grid, s, nr * w + nc, cnt + 1, ans);
            grid[nr][nc] = b'.';
        }
    }
}

fn main() {
    input! {
        h: usize,
        w: usize,
        mut grid: [proconio::marker::Bytes; h],
    }

    let mut ans = 0;
    for r in 0..h {
        for c in 0..w {
            if grid[r][c] == b'.' {
                grid[r][c] = 0;
                solve(&mut grid, r * w + c, r * w + c, 1, &mut ans);
                grid[r][c] = b'.';
            }
        }
    }
    if ans <= 2 {
        println!("-1");
    } else {
        println!("{ans}");
    }
}
