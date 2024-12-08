use proconio::input;

fn main() {
    input! {
        n: usize,
        tv: [(u32, u32); n],
    }
    let mut time = 0;
    let mut w = 0u32;
    for &(ti, vi) in &tv {
        while time < ti {
            time += 1;
            w = w.saturating_sub(1);
        }
        w += vi;
    }
    println!("{w}");
}
