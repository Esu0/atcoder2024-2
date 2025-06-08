use proconio::input;

fn main() {
    input! {
        x: u8,
        y: u8,
    }

    let mut cnt = 0;
    for i in 1u8..=6 {
        for j in 1u8..=6 {
            if i.abs_diff(j) >= y || i + j >= x {
                cnt += 1;
            }
        }
    }
    println!("{}", cnt as f64 / 36.);
}
