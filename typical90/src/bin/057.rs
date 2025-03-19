use proconio::input;
use util::ModInt;
type MInt = ModInt<998244353>;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut mat = vec![vec![false; n]; m];
    for i in 0..n {
        input! { t: usize }
        for _ in 0..t {
            input! { a: usize }
            mat[a - 1][i] = true;
        }
    }
    input! { s: [u8; m] }
    let mut b = s.iter().map(|&c| c == 1).collect::<Vec<_>>();

    let mut j = 0;
    for i in 0..n {
        for k in j..m {
            if mat[k][i] {
                mat.swap(j, k);
                b.swap(j, k);
                for k in j + 1..m {
                    if mat[k][i] {
                        for l in i..n {
                            mat[k][l] ^= mat[j][l];
                        }
                        b[k] ^= b[j];
                    }
                }
                j += 1;
                break;
            }
        }
    }
    // mat.iter().for_each(|row| eprintln!("{row:?}"));
    // eprintln!("{b:?}");
    let mut i = m;
    while i > 0 {
        if mat[i - 1].iter().any(|&c| c) {
            break;
        }
        i -= 1;
        if b[i] {
            println!("0");
            return;
        }
    }
    if i > n {
        println!("0");
    } else {
        let ans = MInt::new(2).pow((n - i) as u32);
        println!("{ans}");
    }
}
