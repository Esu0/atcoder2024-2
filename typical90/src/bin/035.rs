use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n - 1],
        q: usize,
    }
    let mut depth = vec![usize::MAX; n];
    let mut ancestors = vec![vec![]; n];
    let mut stack = Vec::with_capacity(n);
    let mut priorities = vec![usize::MAX; n];
    depth[0] = 0;
    let adj_list = {
        let mut v = vec![vec![]; n];
        for &(a, b) in &ab {
            let a = a - 1;
            let b = b - 1;
            v[a].push(b);
            v[b].push(a);
        }
        v
    };

    let mut p = 0;
    stack.push(0);
    while let Some(u) = stack.pop() {
        priorities[u] = p;
        p += 1;
        for &v in &adj_list[u] {
            if priorities[v] == usize::MAX {
                stack.push(v);
            }
        }
    }
    let adj_list = {
        let mut new = vec![vec![]; n];
        for &(a, b) in &ab {
            let a = priorities[a - 1];
            let b = priorities[b - 1];
            new[a].push(b);
            new[b].push(a);
        }
        new
    };
    stack.push(0);
    while let Some(u) = stack.pop() {
        let depu = depth[u];
        for &v in &adj_list[u] {
            if depth[v] == usize::MAX {
                depth[v] = depu + 1;
                ancestors[v].push(u);
                stack.push(v);
            }
        }
    }

    for i in 0..30 {
        for j in 0..n {
            let Some(k) = ancestors[j].get(i).copied() else {
                continue;
            };
            let Some(l) = ancestors[k].get(i).copied() else {
                continue;
            };
            ancestors[j].push(l);
        }
    }

    // eprintln!("{ancestors:?}");
    // eprintln!("{priorities:?}");
    // eprintln!("{depth:?}");
    for _ in 0..q {
        input! {
            k: usize,
            mut v: [usize; k],
        }
        v.iter_mut().for_each(|vi| *vi = priorities[*vi - 1]);
        v.sort_unstable();
        // eprintln!("{v:?}");
        let mut root = v[0];
        let mut prev = v[0];
        let mut count = 0;
        for &vi in &v[1..] {
            let lca = {
                let (mut a, mut b) = (prev, vi);
                if depth[a] > depth[b] {
                    (a, b) = (b, a);
                }
                let mut dep_diff = depth[b] - depth[a];
                let mut j = 0;
                while dep_diff > 0 {
                    if dep_diff & 1 != 0 {
                        b = ancestors[b][j];
                    }
                    dep_diff >>= 1;
                    j += 1;
                }
                // assert_eq!(depth[a], depth[b]);
                if a != b {
                    for j in (0..ancestors[a].len()).rev() {
                        let Some(&aa) = ancestors[a].get(j) else {
                            continue;
                        };
                        let ab = ancestors[b][j];
                        if aa != ab {
                            a = aa;
                            b = ab;
                        }
                    }
                    assert_eq!(ancestors[a][0], ancestors[b][0]);
                    ancestors[a][0]
                } else {
                    a
                }
            };
            // eprintln!("{lca}");
            // eprintln!("{root} {vi}");
            // eprintln!("{} {} {}", depth[root], depth[vi], depth[lca]);
            if depth[lca] < depth[root] {
                count += depth[root] - depth[lca];
                root = lca;
            }
            count += depth[vi] - depth[lca];
            prev = vi;
        }
        println!("{count}");
    }
}
