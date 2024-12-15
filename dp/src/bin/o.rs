use proconio::input;
use util::ModInt;
type MInt = ModInt<1000000007>;

fn main() {
    input! {
        n: usize,
        a: [[u8; n]; n],
    }
    let mut dp = vec![MInt::new(0); 1 << n];
    dp[0] = MInt::new(1);
    let mut indice = vec![vec![]; n + 1];
    for i in 0..1usize << n {
        indice[i.count_ones() as usize].push(i);
    }

    for (i, row) in a.iter().enumerate() {
        for &j in &indice[i] {
            let dpj = dp[j];
            for (k, &aik) in row.iter().enumerate() {
                if j & (1 << k) == 0 && aik == 1 {
                    dp[j | (1 << k)] += dpj;
                }
            }
        }
    }
    println!("{}", dp[(1 << n) - 1]);
}
