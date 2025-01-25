use proconio::input;
use util::upper_bound;
fn main() {
    input! {
        n: usize,
        m: u64,
        p: [u64; n],
    }

    let k = upper_bound(0u64..1_000_000_000_000_000_000, |x| {
        let mut s = 0u64;
        for &pi in &p {
            let c = (x + pi) / (2 * pi);
            s = s.saturating_add(c.saturating_mul(c).saturating_mul(pi))
        }
        s <= m
    }) - 1;
    let mut rem = m;
    let mut count = 0;
    for &pi in &p {
        let c = (k + pi) / (2 * pi);
        rem -= c * c * pi;
        count += c;
    }
    // eprintln!("{rem}");
    println!("{}", count + rem / (k + 1));
}
