use proconio::input;

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

fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        xy: [(usize, usize); m],
        ab: [(usize, usize); q],
    }

    let expected = solve_naive(n, &xy, &ab);
    expected.iter().for_each(|&ansi| {
        if ansi {
            println!("Yes")
        } else {
            println!("No")
        }
    });
}
