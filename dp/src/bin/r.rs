use proconio::input;
use util::ModInt;
type MInt = ModInt<1000000007>;

fn main() {
    input! {
        n: usize,
        k: u64,
        a: [[u8; n]; n],
    }

    let mut dp = vec![vec![MInt::new(0); n]; n];
    for (i, row) in dp.iter_mut().enumerate() {
        row[i] = MInt::new(1);
    }
    let mut base = a.iter().map(|row| row.iter().map(|&aij| MInt::new(aij as _)).collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut buf1 = Vec::new();
    let mut buf2 = Vec::new();
    let prod = |dst: &mut [Vec<MInt>], a: &[Vec<MInt>], b: &[Vec<MInt>]| {
        for (i, row) in dst.iter_mut().enumerate() {
            for (j, dst) in row.iter_mut().enumerate() {
                let mut sum = MInt::new(0);
                for (k, bk) in b.iter().enumerate() {
                    sum += a[i][k] * bk[j];
                }
                *dst = sum;
            }
        }
    };
    let mut exp = k;
    while exp > 0 {
        if exp & 1 == 1 {
            buf1.clone_from(&dp);
            prod(&mut dp, &buf1, &base);
        }
        buf1.clone_from(&base);
        buf2.clone_from(&base);
        prod(&mut base, &buf1, &buf2);
        exp >>= 1;
    }
    let ans = dp.iter().flat_map(|row| row.iter().copied()).fold(MInt::new(0), |acc, x| acc + x);
    println!("{ans}");
}
