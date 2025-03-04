use proconio::input;

fn main() {
    input! {
        n: usize,
        b: [u32; n],
        r: [u32; n],
    }

    let s = b.iter().copied().sum::<u32>() + r.iter().copied().sum::<u32>();
    println!("{}", s as f64 / n as f64);
}
