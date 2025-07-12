
fn main() {
    proconio::input! {
        n: usize,
        m: usize,
        mut x: [u64; n],
    }
    x.sort_unstable();
    for i in (0..n - 1).rev() {
        x[i + 1] -= x[i];
    }
    x[1..].sort_unstable();
    let ans = x[1..n - m + 1].iter().copied().sum::<u64>();
    println!("{ans}");
}
