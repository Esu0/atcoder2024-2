use proconio::input;

fn main() {
    input! {
        _n: usize,
        m: usize,
        mut uv: [(usize, usize); m],
    }

    uv.iter_mut().for_each(|(u, v)| if *u > *v { std::mem::swap(u, v) });
    uv.sort_unstable();
    uv.dedup();
    let d = uv.iter().filter(|&&(ui, vi)| ui == vi).count();
    println!("{}", m - (uv.len() - d));
}
