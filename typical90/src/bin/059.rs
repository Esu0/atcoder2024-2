use proconio::input;

// fn solve_naive(n: usize, xy: &[(usize, usize)], ab: &[(usize, usize)]) -> Vec<bool> {
//     let mut g = vec![vec![]; n];
//     for &(xi, yi) in xy {
//         g[xi - 1].push(yi - 1);
//     }
//     let mut ans = Vec::with_capacity(ab.len());
//     let mut stack = Vec::new();
//     let mut vis = vec![false; n];
//     for &(ai, bi) in ab {
//         stack.clear();
//         stack.push(ai - 1);
//         vis.fill(false);
//         let mut ansi = false;
//         while let Some(u) = stack.pop() {
//             if vis[u] {
//                 continue;
//             }
//             if u == bi - 1 {
//                 ansi = true;
//                 break;
//             }
//             vis[u] = true;
//             for &v in &g[u] {
//                 if !vis[v] {
//                     stack.push(v);
//                 }
//             }
//         }
//         ans.push(ansi);
//     }
//     ans
// }

fn dfs<const N: usize>(u: usize, g: &[Vec<usize>], dp: &mut [(bool, [u64; N])], offset: usize) {
    if dp[u].0 {
        return;
    }
    let mut s = [0; N];
    if (offset..offset + N * 64).contains(&u) {
        let idx = (u - offset) / 64;
        let rem = (u - offset) % 64;
        s[idx] = 1 << rem;
    }
    for &v in &g[u] {
        if !dp[v].0 {
            dfs(v, g, dp, offset);
        }
        for i in 0..N {
            s[i] |= dp[v].1[i];
        }
    }
    dp[u] = (true, s);
}

fn solve(n: usize, xy: &[(usize, usize)], ab: &[(usize, usize)]) -> Vec<bool> {
    let mut ais = vec![vec![]; n];
    for (i, &(ai, bi)) in ab.iter().enumerate() {
        ais[bi - 1].push((i, ai - 1));
    }
    let mut g = vec![vec![]; n];
    for &(xi, yi) in xy {
        g[xi - 1].push(yi - 1);
    }

    const N: usize = 16;
    const B: usize = N * 64;
    let mut dp = vec![(false, [0; N]); n];
    let mut ans = vec![false; ab.len()];
    for i in 0..(n + B - 1) / B {
        dp.iter_mut().for_each(|x| x.0 = false);
        for u in 0..n {
            dfs(u, &g, &mut dp, i * B);
        }
        for bj in i * B..n.min((i + 1) * B) {
            let idx = (bj - i * B) / 64;
            let sh = bj % 64;
            for &(j, aj) in &ais[bj] {
                if (dp[aj].1[idx] >> sh) & 1 != 0 {
                    ans[j] = true;
                }
            }
        }
    }
    ans
}

fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        xy: [(usize, usize); m],
        ab: [(usize, usize); q],
    }
    // let expected = solve_naive(n, &xy, &ab);
    let actual = solve(n, &xy, &ab);
    // assert_eq!(expected, actual);
    use std::fmt::Write;
    let mut s = String::new();
    for &ans in &actual {
        if ans {
            writeln!(s, "Yes").unwrap();
        } else {
            writeln!(s, "No").unwrap();
        }
    }
    print!("{s}");
}
