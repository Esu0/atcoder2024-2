use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut ans = 0.;
    for _ in 0..n {
        input! { p: u32, q: u32 }
        ans += q as f64 / p as f64;
    }
    println!("{ans}");
}
