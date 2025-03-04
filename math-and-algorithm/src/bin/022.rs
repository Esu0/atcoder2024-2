use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut count = [0u64; 100000];

    let mut ans = 0;
    for &ai in &a {
        ans += count[100000 - ai];
        count[ai] += 1;
    }
    println!("{ans}");
}
