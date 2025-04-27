use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        uvc: [(usize, usize, char); m],
    }

    let mut g = vec![vec![]; n];
    let mut g_rev = vec![vec![]; n];
    let mut sum = 0;
    for &(ui, vi, ci) in &uvc {
        let w = if ci == '(' {
            1
        } else {
            -1
        };
        sum += w;
        g[ui - 1].push((vi - 1, w));
        g_rev[vi - 1].push((ui - 1, w));
    }

    for i in 0..n {
        eprintln!("{}: indeg={}, outdeg={}, diff={}", i + 1, g_rev[i].len(), g[i].len(), g_rev[i].len() as isize - g[i].len() as isize);
    }
    eprintln!("{sum}");
}
