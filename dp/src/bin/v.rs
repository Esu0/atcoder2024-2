use proconio::input;

// fn mpow(mut base: u64, mut exp: u64, m: u64) -> u64 {
//     let mut res = 1;
//     while exp > 0 {
//         if exp & 1 != 0 {
//             res = res * base % m;
//         }
//         base = base * base % m;
//         exp >>= 1;
//     }
//     res
// }

fn dfs1(u: usize, adj_list: &[Vec<usize>], dp: &mut [u64], visited: &mut [bool], m: u64) {
    dp[u] = 1;
    visited[u] = true;
    for &v in &adj_list[u] {
        if !visited[v] {
            dfs1(v, adj_list, dp, visited, m);
            dp[u] = dp[u] * (dp[v] + 1) % m;
        }
    }
}

fn dfs2(u: usize, a: u64, adj_list: &[Vec<usize>], dp: &mut [u64], visited: &mut [bool], buf: &mut [u64], m: u64) {
    dp[u] = dp[u] * (a + 1) % m;
    visited[u] = true;
    let mut p = 1;
    for &v in adj_list[u].iter() {
        if !visited[v] {
            buf[v] = p;
            p = p * (dp[v] + 1) % m;
        }
    }
    let mut prod = 1;
    for &v in adj_list[u].iter().rev() {
        if !visited[v] {
            buf[v] = buf[v] * prod % m;
            prod = prod * (dp[v] + 1) % m;
        }
    }
    for &v in adj_list[u].iter() {
        if !visited[v] {
            dfs2(v, buf[v] * (a + 1) % m, adj_list, dp, visited, buf, m);
        }
    }
}
fn main() {
    input! {
        n: usize,
        m: u64,
        xy: [(usize, usize); n - 1],
    }
    let mut adj_list = vec![vec![]; n];
    for &(x, y) in &xy {
        adj_list[x - 1].push(y - 1);
        adj_list[y - 1].push(x - 1);
    }

    let mut dp = vec![0; n];
    let mut buf = vec![0; n];
    let mut visited = vec![false; n];
    dfs1(0, &adj_list, &mut dp, &mut visited, m);
    visited.fill(false);
    // eprintln!("{dp:?}");
    dfs2(0, 0, &adj_list, &mut dp, &mut visited, &mut buf, m);
    for &dpi in dp.iter() {
        println!("{}", dpi);
    }
}
