use proconio::input;

fn main() {
    input! {
        n: u64,
    }

    let mut r = 1;
    let mut cnt = 0;
    while r * 2 <= n {
        r <<= 1;
        cnt += 1;
    }
    println!("{}", cnt * n + (n - r) * 2);
}
