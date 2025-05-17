use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
    }
    use std::collections::HashSet;
    let mut rows = vec![HashSet::new(); h];
    let mut cols = vec![HashSet::new(); w];

    for _ in 0..n {
        input! { x: usize, y: usize }
        rows[x - 1].insert(y - 1);
        cols[y - 1].insert(x - 1);
    }

    input! { q: usize }
    use std::fmt::Write;
    let mut out_s = String::new();
    for _ in 0..q {
        input! { t: u8, x: usize }
        if t == 1 {
            writeln!(out_s, "{}", rows[x - 1].len()).unwrap();
            for &y in &rows[x - 1] {
                cols[y].remove(&(x - 1));
            }
            rows[x - 1].clear();
        } else {
            let y = x;
            writeln!(out_s, "{}", cols[y - 1].len()).unwrap();
            for &x in &cols[y - 1] {
                rows[x].remove(&(y - 1));
            }
            cols[y - 1].clear();
        }
    }
    print!("{out_s}");
}

