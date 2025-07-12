use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut uv: [(usize, usize); m],
        q: usize,
    }

    let mut g = vec![std::collections::HashSet::new(); n];
    uv.iter_mut().for_each(|x| {
        x.0 -= 1;
        x.1 -= 1;
    });
    for &(ui, vi) in &uv {
        g[ui].insert(vi);
        g[vi].insert(ui);
    }
    let mut ans = m;
    let mut vs = (0..n).map(|i| vec![i]).collect::<Vec<_>>();
    let mut vs_rev = (0..n).collect::<Vec<_>>();
    for _ in 0..q {
        input! { x: usize }
        let (mut ux, mut vx) = uv[x - 1];
        ux = vs_rev[ux];
        vx = vs_rev[vx];
        if ux == vx {
            println!("{ans}");
            continue;
        }
        if g[ux].len() > g[vx].len() {
            (ux, vx) = (vx, ux);
        }

        let new_u = if vs[ux].len() > vs[vx].len() { vx } else { ux };
        for &w in &g[vx] {

        }
    }
}
