use proconio::input;
use util::ModInt;
type MInt = ModInt::<1000000007>;

fn main() {
    input! {
        n: usize,
        a: [[u32; 6]; n],
    }

    let mut ans = MInt::new(1);
    for ai in &a {
        let sum = ai.iter().copied().sum::<u32>();
        ans *= MInt::new(sum as _);
    }

    println!("{ans}");
}
