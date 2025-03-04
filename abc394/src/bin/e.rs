use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        c: [proconio::marker::Bytes; n],
    }

    let mut dp = vec![vec![usize::MAX; n]; n];
    let mut queue = VecDeque::new();
    for i in 0..n {
        queue.push_back((0usize, i, i));
    }
    for i in 0..n {
        for j in 0..n {
            if i != j && c[i][j] != b'-' {
                queue.push_back((1usize, i, j));
            }
        }
    }

    while let Some((cost, i, j)) = queue.pop_front() {
        if dp[i][j] != usize::MAX {
            continue;
        }
        dp[i][j] = cost;
        for k in 0..n {
            if c[k][i] == b'-' {
                continue;
            }
            for l in 0..n {
                if c[k][i] == c[j][l] && dp[k][l] == usize::MAX {
                    queue.push_back((cost + 2, k, l));
                }
            }
        }
    }

    for row in &dp {
        for &cell in row {
            print!("{} ", cell as isize);
        }
        println!();
    }
}
