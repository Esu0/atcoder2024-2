use proconio::input;

fn dfs(u: usize, adj_list: &[Vec<usize>], child: &mut [usize]) {
    let mut sum = 0;
    child[u] = 0;
    for &v in &adj_list[u] {
        if child[v] == usize::MAX {
            dfs(v, adj_list, child);
            sum += child[v] + 1;
        }
    }
    child[u] = sum;
}

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n - 1],
    }

    let adj_list = {
        let mut v = vec![vec![]; n];
        for &(a, b) in &ab {
            v[a - 1].push(b - 1);
            v[b - 1].push(a - 1);
        }
        v
    };
    let mut childs = vec![usize::MAX; n];
    dfs(0, &adj_list, &mut childs);
    let mut ans = 0;
    // eprintln!("{childs:?}");
    for &ch in &childs[1..] {
        ans += (ch + 1) * (n - ch - 1);
    }
    println!("{ans}");
}
