use proconio::input;
use util::ModInt;
type MInt = ModInt<998244353>;

fn main() {
    input! {
        n: usize,
        s: usize,
        a: [usize; n],
    }

    let mut dpi = vec![MInt::new(0); s + 1];
    dpi[0] = MInt::new(1);
    let mut dpi_nxt = Vec::new();
    for &ai in &a {
        dpi_nxt.clone_from(&dpi);
        for j in 0..=s {
            dpi_nxt[j] *= 2u32;
        }
        for j in 0..=s {
            if j + ai <= s {
                dpi_nxt[j + ai] += dpi[j];
            }
        }
        dpi.clone_from(&dpi_nxt);
    }
    println!("{}", dpi[s]);
}
