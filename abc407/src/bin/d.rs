use proconio::input;

fn solve(grid: &mut [Vec<bool>], a: &[Vec<u64>], i: usize, j: usize, xor: u64, ans: &mut u64) {
    // eprintln!("{i} {j}");
    assert_eq!((i + j) % 2, 0);
    let h = grid.len();
    let w = grid[0].len();
    if i >= h {
        *ans = xor.max(*ans);
        return;
    }
    let mut next_i = i;
    let mut next_j = j;
    loop {
        next_j += 1;
        if next_j == w {
            next_i += 1;
            next_j = 0;
        }
        if (next_j + next_i) % 2 == 0 {
            break;
        }
    }
    grid[i][j] = true;
    for (di, dj) in [(1, 0), (usize::MAX, 0), (0, 1), (0, usize::MAX)] {
        let ni = i.wrapping_add(di);
        let nj = j.wrapping_add(dj);
        if ni < h && nj < w && !grid[ni][nj] {
            grid[ni][nj] = true;
            solve(grid, a, next_i, next_j, xor ^ a[ni][nj] ^ a[i][j], ans);
            grid[ni][nj] = false;
        }
    }
    grid[i][j] = false;
    solve(grid, a, next_i, next_j, xor, ans);
}

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[u64; w]; h],
    }

    let mut ans = 0;
    let xor = a.iter().flat_map(|ai| ai.iter().copied()).reduce(|acc, x| acc ^ x).unwrap();
    let mut grid = vec![vec![false; w]; h];
    solve(&mut grid, &a, 0, 0, xor, &mut ans);
    println!("{ans}");
}
