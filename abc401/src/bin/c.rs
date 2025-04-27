use proconio::input;
type MInt = util::ModInt<1000000000>;

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let mut cumsum = (0..=k as i64).map(MInt::new).collect::<Vec<_>>();
    while cumsum.len() <= n + 1 {
        let length = cumsum.len();
        let new = cumsum[length - 1] - cumsum[length - 1 - k] + cumsum[length - 1];
        cumsum.push(new);
    }
    println!("{}", cumsum[n + 1] - cumsum[n]);
}
