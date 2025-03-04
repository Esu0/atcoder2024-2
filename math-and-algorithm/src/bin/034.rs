use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(u32, u32); n],
    }

    let mut mn = u64::MAX;
    for (i, &(xi, yi)) in xy.iter().enumerate() {
        for &(xj, yj) in &xy[i + 1..] {
            let dx = xi.abs_diff(xj) as u64;
            let dy = yi.abs_diff(yj) as u64;
            mn = mn.min(dx * dx + dy * dy)
        }
    }
    println!("{}", (mn as f64).sqrt());
}
