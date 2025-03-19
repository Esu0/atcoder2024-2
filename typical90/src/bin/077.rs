use std::collections::HashMap;

use proconio::input;

fn dfs(
    u: usize,
    t: usize,
    ctx: &mut [usize],
    graph: &mut [Vec<(usize, usize, usize, usize)>],
    depth: &[usize],
    f: usize,
) -> usize {
    if u == t {
        return f;
    }

    while ctx[u] < graph[u].len() {
        let ctxu = ctx[u];
        let (v, w, rev, _) = graph[u][ctx[u]];
        if depth[u] < depth[v] && w > 0 {
            let f2 = dfs(v, t, ctx, graph, depth, f.min(w));
            if f2 > 0 {
                graph[u][ctxu].1 -= f2;
                graph[v][rev].1 += f2;
                return f2;
            }
        }
        ctx[u] += 1;
    }
    0
}

fn main() {
    input! {
        n: usize,
        t: i64,
        axy: [(i64, i64); n],
        bxy: [(i64, i64); n],
    }

    let mut g = vec![vec![]; 2 * n + 2];

    let start = 2 * n;
    let end = 2 * n + 1;
    for i in 0..n {
        let i0 = g[start].len();
        let i1 = g[i].len();
        g[start].push((i, 1, i1, 0));
        g[i].push((start, 0, i0, 0));
    }
    for i in n..2 * n {
        let i0 = g[i].len();
        let i1 = g[end].len();
        g[i].push((end, 1, i1, 0));
        g[end].push((i, 0, i0, 0));
    }

    let s = bxy.iter().copied().enumerate().map(|(i, p)| (p, i)).collect::<HashMap<_, _>>();
    let dxy = [(t, 0), (t, t), (0, t), (-t, t), (-t, 0), (-t, -t), (0, -t), (t, -t)];
    for (i, &(ax, ay)) in axy.iter().enumerate() {
        for (k, &(dx, dy)) in dxy.iter().enumerate() {
            if let Some(&j) = s.get(&(ax + dx, ay + dy)) {
                let i0 = g[i].len();
                let i1 = g[n + j].len();
                g[i].push((n + j, 1, i1, k + 1));
                g[n + j].push((i, 0, i0, 0));
            }
        }
    }

    let mut depth = vec![usize::MAX; 2 * n + 2];
    let mut queue = std::collections::VecDeque::new();
    let mut ctx = vec![0; 2 * n + 2];
    let mut flow_val = 0;
    loop {
        depth.fill(usize::MAX);
        depth[start] = 0;
        queue.clear();
        queue.push_back(start);
        while let Some(u) = queue.pop_front() {
            let depu = depth[u];
            for &(v, w, _, _) in &g[u] {
                if w > 0 && depth[v] == usize::MAX {
                    depth[v] = depu + 1;
                    queue.push_back(v);
                }
            }
        }

        if depth[end] == usize::MAX {
            break;
        }

        ctx.fill(0);
        while {
            let diff = dfs(start, end, &mut ctx, &mut g, &depth, usize::MAX);
            flow_val += diff;
            diff > 0
        } {}
    }

    if flow_val != n {
        println!("No");
        return;
    }

    let mut ans = vec![usize::MAX; n];
    for u in 0..n {
        for &(v, w, _, k) in &g[u] {
            if (n..2 * n).contains(&v) && w == 0 {
                assert_eq!(ans[u], usize::MAX);
                ans[u] = k;
            }
        }
    }

    println!("Yes");
    print!("{}", ans[0]);
    for &ai in &ans[1..] {
        print!(" {}", ai);
    }
    println!();
}
