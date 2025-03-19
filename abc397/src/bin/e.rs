use proconio::input;

fn dfs(g: &[Vec<usize>], vis: &mut [bool], u: usize, k: usize) -> Result<usize, ()> {
    vis[u] = true;
    let mut cnt = 0;
    let mut ret = 1usize;
    for &v in &g[u] {
        if !vis[v] {
            let res = dfs(g, vis, v, k)?;
            if res != k {
                ret += res;
                cnt += 1;
            }
        }
    }
    // eprintln!("{} {} {} {}", u, ret, cnt, k);
    if cnt == 0 {
        return Ok(ret);
    }
    if cnt == 1 {
        return Ok(ret);
    }
    if cnt == 2 {
        return if ret != k {
            Err(())
        } else {
            Ok(k)
        };
    }
    Err(())
}

fn main() {
    input! {
        n: usize,
        k: usize,
        uv: [(usize, usize); n * k - 1],
    }

    if k == 1 {
        println!("Yes");
        return;
    }

    let mut vis = vec![false; n * k];
    let mut g = vec![vec![]; n * k];
    for &(ui, vi) in &uv {
        g[ui - 1].push(vi - 1);
        g[vi - 1].push(ui - 1);
    }

    let res = dfs(&g, &mut vis, 0, k);

    if let Ok(x) = res {
        assert_eq!(x, k);
        println!("Yes");
    } else {
        println!("No");
    }
    // eprintln!("{dp:?}");
}
