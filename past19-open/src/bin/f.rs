use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        x: String,
        y: String,
        st: [(String, String); n],
    }
    let mut map = HashMap::new();
    let add = |map: &mut HashMap<String, usize>, key: String| -> usize {
        if let Some(ind) = map.get(&key) {
            *ind
        } else {
            let new = map.len();
            map.insert(key, new);
            new
        }
    };
    let xid = add(&mut map, x);
    let yid = add(&mut map, y);

    let mut edges = Vec::with_capacity(n);
    for (si, ti) in st {
        let sid = add(&mut map, si);
        let tid = add(&mut map, ti);
        edges.push((sid, tid));
    }

    let vs = map.len();
    // eprintln!("{map:?}");
    let mut g = vec![vec![]; vs];
    for &(u, v) in &edges {
        g[u].push(v);
    }

    let mut visited = vec![false; vs];
    let mut stack = vec![xid];
    while let Some(u) = stack.pop() {
        if visited[u] {
            continue;
        }
        if u == yid {
            println!("Yes");
            return;
        }
        visited[u] = true;
        for &v in &g[u] {
            if !visited[v] {
                stack.push(v);
            }
        }
    }
    println!("No");
}
