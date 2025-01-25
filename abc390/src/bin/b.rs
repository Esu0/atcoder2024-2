use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }

    let q = a[0];
    let r = a[1];
    for i in 1..n - 1 {
        // q / r != a[i] / a[i + 1]
        if q * a[i + 1] != a[i] * r {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
