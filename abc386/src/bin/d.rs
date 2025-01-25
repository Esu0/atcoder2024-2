use proconio::input;

fn main() {
    input! {
        _: usize,
        m: usize,
        mut xyc: [(usize, usize, char); m],
    }

    xyc.sort_unstable_by_key(|xyci| (xyci.0, xyci.1));
    let mut jmin = usize::MAX;
    for &(_, yi, ci) in &xyc {
        if ci == 'B' {
            if jmin <= yi {
                println!("No");
                return;
            }
        } else {
            jmin = jmin.min(yi);
        }
    }
    println!("Yes");
}
