use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut g = vec![vec![]; n];
    for _ in 0..m {
        input! { u: usize, v: usize }
        g[u - 1].push(v - 1);
        g[v - 1].push(u - 1);
    }

    let mut ans = 0;
    for u in 0..n {
        let mut cnt = 0;
        for &v in &g[u] {
            if v < u {
                cnt += 1;
            }
        }
        if cnt == 1 {
            ans += 1;
        }
    }
    println!("{ans}");
}
