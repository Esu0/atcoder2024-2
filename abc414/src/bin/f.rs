// use std::collections::VecDeque;

use proconio::input;


fn main() {
    input! {
        t: usize,
    }

    let mut stack0 = Vec::new();
    let mut stack = Vec::new();
    for _ in 0..t {
        input! {
            n: usize,
            k: usize,
            uv: [(usize, usize); n - 1],
        }

        let mut g = vec![vec![]; n];
        let mut dp = vec![vec![]; n];
        let mut cnt = vec![vec![0u8; n]; k];
        for &(ui, vi) in &uv {
            g[ui - 1].push(vi - 1);
            g[vi - 1].push(ui - 1);
            dp[ui - 1].push(vec![usize::MAX; k]);
            dp[vi - 1].push(vec![usize::MAX; k]);
        }

        stack0.push((0, usize::MAX, 0, 0));
        // for (dp0v, &v) in dp[0].iter_mut().zip(&g[0]) {
        //     dp0v[1] = 0;
        //     stack0.push((1, 0, v, 0));
        //     cnt[1][v] += 1;
        // }

        while let Some((i, u, v, d)) = stack0.pop().or_else(|| stack.pop()) {
            let ni = (i + 1) % k;
            for (dpvj, &w) in dp[v].iter_mut().zip(&g[v]) {
                if dpvj[ni] == usize::MAX {
                    if i == 0 {
                        cnt[ni][w] += 1;
                        if cnt[ni][w] == 3 {
                            continue;
                        }
                        dpvj[ni] = d + 1;
                        stack.push((ni, v, w, d + 1));
                    } else if u != w {
                        cnt[ni][w] += 1;
                        if cnt[ni][w] == 3 {
                            continue;
                        }
                        dpvj[ni] = d;
                        stack0.push((ni, v, w, d));
                    }
                }
            }
            dp.iter().zip(&g).enumerate().for_each(|(i, (dpi, gi))| {
                dpi.iter().zip(gi).for_each(|(dpij, &v)| eprintln!("({i}, {v}): {dpij:?}"));
            });
            // eprintln!("{:?}", dp[4]);
        }

        let mut ans = vec![usize::MAX; n];
        for (dpu, gu) in dp.iter().zip(&g) {
            for (dpuj, &v) in dpu.iter().zip(gu) {
                ans[v] = ans[v].min(dpuj[0]);
            }
        }

        print!("{}", ans[1] as isize);
        for &ansi in &ans[2..] {
            print!(" {}", ansi as isize);
        }
        println!();
    }
}
