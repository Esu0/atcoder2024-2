use proconio::input;
use util::upper_bound;

fn main() {
    input! {
        n: u64,
    }
    let mut a = 1;
    let mut ans = 0;
    for _ in 1..64 {
        a <<= 1;
        if n < a {
            break;
        }
        let k = upper_bound(0u128..600_000_000, |x| {
            let b = 2 * x + 1;
            a as u128 * b * b <= n as u128
        });
        eprintln!("{k}");
        ans += k as u64;
    }
    println!("{ans}");
}
