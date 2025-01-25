use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }

    let mut ans = 0;
    for i in 0..n {
        let p = a.partition_point(|&x| x < 2 * a[i]);
        ans += n - p;
    }
    println!("{ans}");
}
