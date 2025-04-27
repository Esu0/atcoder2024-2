use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut mat = vec![vec![0; n]; n];
    for i in 0..n {
        for j in i + 1..n {
            input! { a: u64 }
            mat[i][j] = a;
            mat[j][i] = a;
        }
    }

    // eprintln!("{mat:?}");
    let mut b = vec![vec![0; n]; n];
    for i in 0..n {
        for j in i + 1..n {
            input! { bij: u64 }
            b[i][j] = bij;
            b[j][i] = bij;
        }
    }

    let mut dist = vec![vec![[u64::MAX; 2]; n]; n];
    let mut vis = vec![[false; 2]; n];
    for s in 0..n {
        let dists = &mut dist[s];
        dists[s][0] = 0;
        vis.fill([false; 2]);
        loop {
            let mut min = u64::MAX;
            let mut argmin = (usize::MAX, usize::MAX);
            for i in 0..n {
                for j in 0..2 {
                    if !vis[i][j] && dists[i][j] < min {
                        min = dists[i][j];
                        argmin = (i, j);
                    }
                }
            }
            if argmin.0 == usize::MAX {
                break;
            }
            vis[argmin.0][argmin.1] = true;
            for j in 0..n {
                dists[j][argmin.1] = dists[j][argmin.1].min(min + mat[argmin.0][j]);
            }
            if argmin.1 == 0 {
                for j in 0..n {
                    dists[j][1] = dists[j][1].min(min + b[argmin.0][j]);
                }
            }
        }
    }

    for i in 0..n {
        for j in i + 1..n {
            let ans = dist[i][j][0].min(dist[i][j][1]);
            print!("{ans}");
            if j == n - 1 {
                println!();
            } else {
                print!(" ");
            }
        }
    }
}
