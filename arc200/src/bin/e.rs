use proconio::input;
type MInt = util::ModInt<998244353>;
use std::fmt::Write;
fn main() {
    input! {
        t: usize,
    }

    let mut output = String::new();
    let inv2 = MInt::new(2).inv();
    let inv6 = MInt::new(6).inv();
    for _ in 0..t {
        input! { n: u32, m: u32 }
        let p2m = MInt::new(2).pow(m);
        let m = MInt::new(m as _);
        let pm = (m + MInt::new(1)).pow(n - 1);
        let p2 = MInt::new(2).pow(n - 1);
        let p3 = MInt::new(3).pow(n - 1);
        let p4 = MInt::new(4).pow(n - 1);
        let c2 = m * (m - MInt::new(1)) * inv2;
        let c3 = m * (m - MInt::new(1)) * (m - MInt::new(2)) * inv6;
        let ans = (pm + c2 * (p4 - p3) + c3 * (p4 - MInt::new(3) * p3 + MInt::new(3) * p2 - MInt::new(1)) + m * (pm - p2 - (m - MInt::new(1)) * (p3 - p2))) * p2m;

        writeln!(&mut output, "{ans}").unwrap();
    }
    print!("{output}");
}
