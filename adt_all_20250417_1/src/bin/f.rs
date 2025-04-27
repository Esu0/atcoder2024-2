use proconio::input;

fn main() {
    input! {
        n: usize,
        d: i64,
        xy: [(i64, i64); n],
    }

    let mut dist = vec![vec![0; n]; n];
    for (i, &(xi, yi)) in xy.iter().enumerate() {
        for (j, &(xj, yj)) in xy[..i].iter().enumerate() {
            dist[i][j] = (xi - xj).pow(2) + (yi - yj).pow(2);
            dist[j][i] = dist[i][j];
        }
    }

    let d = d * d;
    let mut stack = vec![0];
    let mut vis = vec![false; n];
    while let Some(u) = stack.pop() {
        if vis[u] {
            continue;
        }
        vis[u] = true;
        for v in 0..n {
            if !vis[v] && v != u && dist[u][v] <= d {
                stack.push(v);
            }
        }
    }
    for &ans in &vis {
        if ans {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
