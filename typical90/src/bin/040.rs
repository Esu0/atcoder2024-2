use std::collections::{HashMap, VecDeque};

use proconio::input;

fn dfs(
    u: usize,
    t: usize,
    itr: &mut [usize],
    depth: &[usize],
    adj_list: &[Vec<usize>],
    weights: &mut HashMap<(usize, usize), u64>,
    f: u64,
) -> u64 {
    if u == t {
        return f;
    }
    while itr[u] < adj_list[u].len() {
        let v = adj_list[u][itr[u]];
        let &e = weights.get(&(u, v)).unwrap();
        if depth[v] > depth[u] && e > 0 {
            let f2 = dfs(v, t, itr, depth, adj_list, weights, f.min(e));
            if f2 > 0 {
                *weights.get_mut(&(u, v)).unwrap() -= f2;
                *weights.get_mut(&(v, u)).unwrap() += f2;
                return f2;
            }
        }
        itr[u] += 1;
    }
    0
}

fn main() {
    input! {
        n: usize,
        w: u64,
        a: [u64; n],
        c: [[usize]; n],
    }
    let v_count = 2 * n + 2;
    let mut adj_list = vec![vec![]; v_count];
    let mut weights = HashMap::new();
    let s = 2 * n;
    let t = 2 * n + 1;
    let a_max = a.iter().copied().max().unwrap().max(w);
    for (i, &ai) in a.iter().enumerate() {
        adj_list[2 * i].push(2 * i + 1);
        weights.insert((2 * i, 2 * i + 1), a_max - ai);
        adj_list[2 * i + 1].push(2 * i);
        weights.insert((2 * i + 1, 2 * i), 0);
        adj_list[s].push(2 * i);
        weights.insert((s, 2 * i), u64::MAX);
        adj_list[2 * i].push(s);
        weights.insert((2 * i, s), 0);
        adj_list[2 * i + 1].push(t);
        weights.insert((2 * i + 1, t), a_max - w);
        adj_list[t].push(2 * i + 1);
        weights.insert((t, 2 * i + 1), 0);
    }
    for (i, ci) in c.iter().enumerate() {
        for &cij in ci {
            let cij = cij - 1;
            adj_list[2 * i + 1].push(2 * cij + 1);
            weights.insert((2 * i + 1, 2 * cij + 1), u64::MAX);
            adj_list[2 * cij + 1].push(2 * i + 1);
            weights.insert((2 * cij + 1, 2 * i + 1), 0);
        }
    }
    let mut depth = vec![usize::MAX; v_count];
    let mut itr = vec![0; v_count];
    let mut queue = VecDeque::new();
    let mut flow = 0;

    loop {
        // bfs
        queue.push_back(s);
        depth.fill(usize::MAX);
        depth[s] = 0;
        while let Some(u) = queue.pop_front() {
            let depu = depth[u];
            for &v in &adj_list[u] {
                let e = weights[&(u, v)];
                if e != 0 && depth[v] == usize::MAX {
                    depth[v] = depu + 1;
                    queue.push_back(v);
                }
            }
        }
        if depth[t] == usize::MAX {
            break;
        }
        itr.fill(0);

        loop {
            let diff = dfs(s, t, &mut itr, &depth, &adj_list, &mut weights, u64::MAX);
            if diff == 0 {
                break;
            }
            flow += diff;
        }
    }

    // {
    //     let mut edges = vec![];
    //     for (i, e) in adj_list.iter().enumerate() {
    //         for &j in e {
    //             let w = *weights.get(&(i, j)).unwrap();
    //             if w > 0 {
    //                 edges.push((i, j, w));
    //             }
    //         }
    //     }
    //     println!("{} {}", v_count, edges.len());
    //     edges.iter().for_each(|&(a, b, c)| println!("{a} {b} {c}"));
    // }
    // eprintln!("{flow}");
    let ans = (a_max - w) * n as u64 - flow;

    // {
    //     let mut keys = vec![vec![]; n];
    //     for (i, ci) in c.iter().enumerate() {
    //         for &cij in ci {
    //             keys[cij - 1].push(i);
    //         }
    //     }
    //     let mut max = 0;
    //     for b in 0..(1 << n) {
    //         if (0..n).all(|i| {
    //             b & (1 << i) == 0 || keys[i].iter().all(|&j| b & (1 << j) != 0)
    //         }) {
    //             let mut sum = 0;
    //             for i in 0..n {
    //                 if b & (1 << i) != 0 {
    //                     sum += a[i] as i64 - w as i64;
    //                 }
    //             }
    //             max = max.max(sum);
    //         }
    //     }
    //     assert_eq!(ans, max as u64);
    // }
    println!("{ans}");
}
