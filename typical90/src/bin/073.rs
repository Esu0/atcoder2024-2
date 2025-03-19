use proconio::input;
type MInt = util::ModInt<1000000007>;

fn dfs(g: &[Vec<usize>], c: &[char], u: usize, vis: &mut [bool]) -> (MInt, MInt, MInt) {
    vis[u] = true;
    let mut ret = (MInt::new(0), MInt::new(0), MInt::new(0));
    if c[u] == 'a' {
        ret.1 = MInt::new(1);
    } else {
        assert_eq!(c[u], 'b');
        ret.2 = MInt::new(1);
    }

    for &v in &g[u] {
        if !vis[v] {
            let res = dfs(g, c, v, vis);
            let nxt = (
                ret.0 * res.0 + (ret.0 + ret.1 + ret.2) * res.0 + (ret.0 + ret.2) * res.1 + (ret.0 + ret.1) * res.2,
                ret.1 * res.0 + ret.1 * res.1,
                ret.2 * res.0 + ret.2 * res.2,
            );
            ret = nxt;
        }
    }
    ret
}

fn main() {
    input! {
        n: usize,
        c: [char; n],
        ab: [(usize, usize); n - 1],
    }

    let mut g = vec![vec![]; n];
    for &(ai, bi) in &ab {
        g[ai - 1].push(bi - 1);
        g[bi - 1].push(ai - 1);
    }
    let mut vis = vec![false; n];
    let res = dfs(&g, &c, 0, &mut vis);
    println!("{}", res.0);
}
