use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
        b: [u32; n],
    }
    let ans = (a.iter().copied().sum::<u32>() + 2 * b.iter().copied().sum::<u32>()) as f64 / 3.;
    println!("{ans}");
}
