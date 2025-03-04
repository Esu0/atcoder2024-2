use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut ans = 0.;
    for i in 0..n {
        ans += n as f64 / (n - i) as f64;
    }
    println!("{ans}");
}