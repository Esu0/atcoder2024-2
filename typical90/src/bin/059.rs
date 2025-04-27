use proconio::input;
use rand::seq::SliceRandom;

fn solve_naive(n: usize, xy: &[(usize, usize)], ab: &[(usize, usize)]) -> Vec<bool> {
    let mut g = vec![vec![]; n];
    for &(xi, yi) in xy {
        g[xi - 1].push(yi - 1);
    }
    let mut ans = Vec::with_capacity(ab.len());
    let mut stack = Vec::new();
    let mut vis = vec![false; n];
    for &(ai, bi) in ab {
        stack.clear();
        stack.push(ai - 1);
        vis.fill(false);
        let mut ansi = false;
        while let Some(u) = stack.pop() {
            if vis[u] {
                continue;
            }
            if u == bi - 1 {
                ansi = true;
                break;
            }
            vis[u] = true;
            for &v in &g[u] {
                if !vis[v] {
                    stack.push(v);
                }
            }
        }
        ans.push(ansi);
    }
    ans
}

fn dfs(g: &[Vec<usize>], ais: &mut [Vec<(usize, usize)>], ans: &mut [u8], u: usize, vis: &mut [u8]) {
    assert_eq!(vis[u], 0);
    vis[u] = 1;
    for &v in &g[u] {
        if vis[v] == 0 {
            dfs(g, ais, ans, v, vis);
        }
    }
    for &(ai, i) in &ais[u] {
        let yn = if vis[ai] == 1 {
            b'Y'
        } else if vis[ai] == 2 {
            b'N'
        } else {
            b'?'
        };
        if yn != b'?' {
            if ans[i] != b'?' {
                assert_eq!(ans[i], yn);
            } else {
                ans[i] = yn;
            }
        }
    }
    // ais[u].retain(|&(ai, _)| vis[ai] == 0);
    vis[u] = 2;
}

fn solve(g: &mut [Vec<usize>], ab: &[(usize, usize)]) -> Vec<u8> {
    let n = g.len();
    let q = ab.len();
    let mut ais = vec![vec![]; n];
    let mut ans = vec![b'?'; q];
    let mut vis = vec![0u8; n];
    for (i, &(ai, bi)) in ab.iter().enumerate() {
        ais[bi - 1].push((ai - 1, i));
    }

    for u in 0..n {
        if vis[u] == 0 {
            dfs(g, &mut ais, &mut ans, u, &mut vis);
        } else {
            assert_eq!(vis[u], 2);
        }
    }

    // for gi in &mut *g {
    //     gi.reverse();
    // }
    // let mut rng = rand::thread_rng();
    // let mut ord = (0..n).collect::<Vec<_>>();
    // for _ in 0..500 {
    //     ord.shuffle(&mut rng);
    //     vis.fill(0u8);
    //     for &u in &ord {
    //         if vis[u] == 0 {
    //             dfs(g, &mut ais, &mut ans, u, &mut vis);
    //         } else {
    //             assert_eq!(vis[u], 2);
    //         }
    //     }
    // }
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

    let mut g = vec![vec![]; n];
    for &(xi, yi) in &xy {
        g[xi - 1].push(yi - 1);
    }
    let expected = solve_naive(n, &xy, &ab);
    let actual = solve(&mut g, &ab);
    eprintln!("{}", std::str::from_utf8(&actual).unwrap());
    expected.iter().zip(&actual).for_each(|(&ansi, &act)| {
        if ansi {
            if act != b'?' {
                assert_eq!(act, b'Y');
            }
            println!("Yes")
        } else {
            if act != b'?' {
                assert_eq!(act, b'N');
            }
            println!("No")
        }
    });

    let mut cnt = 0;
    for (i, &(ai, bi)) in ab.iter().enumerate() {
        if actual[i] == b'?' {
            cnt += 1;
            println!("{ai} {bi}: {}", if expected[i] { "Yes" } else { "No" });
        }
    }
    eprintln!("{cnt}");
}
