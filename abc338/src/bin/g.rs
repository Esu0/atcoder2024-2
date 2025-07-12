use proconio::input;
type MInt = util::ModInt<998244353>;

fn main() {
    input! {
        s: proconio::marker::Bytes,
    }

    let mut a = MInt::new(0);
    let mut ab = MInt::new(0);
    let mut i = 0;
    let mut j = 0;
    let mut x = MInt::new(0);
    let mut ans = MInt::new(0);
    while j < s.len() {
        i += 1;
        if s[j] == b'+' {
            x += ab;
            a = MInt::new(i);
            j += 1;
            ab = MInt::new((s[j] - b'0') as _) * a;
        } else if s[j] == b'*' {
            a = ab + MInt::new(1);
            j += 1;
            ab = MInt::new((s[j] - b'0') as _) * a;
        } else {
            a += MInt::new(1);
            ab = MInt::new(10) * ab + MInt::new((s[j] - b'0') as _) * a;
        }
        ans += x + ab;
        j += 1;
    }
    println!("{ans}");
}
