use std::collections::VecDeque;

use proconio::input;
#[derive(Clone, Copy)]
enum State {
    None,
    Stopped,
    Min(usize),
}
fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        rc: [(usize, usize); k],
    }

    let mut dp = vec![vec![State::None; w]; h];

    let mut queue = VecDeque::new();
    for &(ri, ci) in &rc {
        dp[ri - 1][ci - 1] = State::Min(0);
        queue.push_back((ri - 1, ci - 1));
    }

    let drc = [(0, 1), (0, usize::MAX), (1, 0), (usize::MAX, 0)];
    while let Some((r, c)) = queue.pop_front() {
        let State::Min(dprc) = dp[r][c] else {
            panic!();
        };
        for (dr, dc) in drc {
            let nr = r.wrapping_add(dr);
            let nc = c.wrapping_add(dc);
            if nr < h && nc < w {
                match dp[nr][nc] {
                    State::Min(_) => {}
                    State::None => dp[nr][nc] = State::Stopped,
                    State::Stopped => {
                        dp[nr][nc] = State::Min(dprc + 1);
                        queue.push_back((nr, nc));
                    }
                }
            }
        }
    }

    let mut ans = 0;
    for dpi in &dp {
        for &dpij in dpi {
            if let State::Min(x) = dpij {
                ans += x;
            }
        }
    }
    println!("{ans}");
}
