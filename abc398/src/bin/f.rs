use proconio::{input, marker};
use util::ModInt;
type MInt1 = ModInt<998244353>;
type MInt2 = ModInt<1000000007>;

fn main() {
    input! {
        s: marker::Bytes,
    }
    use rand::Rng;

    let mut rng = rand::thread_rng();
    let b1 = MInt1::new(rng.gen_range(2..998244352));
    let b2 = MInt2::new(rng.gen_range(2..1000000006));
    let mut hash1 = Vec::with_capacity(s.len() + 1);
    let mut hash2 = Vec::with_capacity(s.len() + 1);
    let mut h1 = MInt1::new(0);
    let mut h2 = MInt2::new(0);
    hash1.push(h1);
    hash2.push(h2);
    for &si in &s {
        h1 = b1 * h1 + MInt1::new(si as _);
        h2 = b2 * h2 + MInt2::new(si as _);
        hash1.push(h1);
        hash2.push(h2);
    }

    let mut hash_rev1 = vec![MInt1::new(0); s.len() + 1];
    let mut hash_rev2 = vec![MInt2::new(0); s.len() + 1];
    // let mut pb1 = MInt1::new(1);
    // let mut pb2 = MInt2::new(1);

    for i in (0..s.len()).rev() {
        hash_rev1[i] = hash_rev1[i + 1] * b1 + MInt1::new(s[i] as _);
        hash_rev2[i] = hash_rev2[i + 1] * b2 + MInt2::new(s[i] as _);
    }
    let n = s.len();
    for i in 0..n {
        let l = n - i;
        let half = l / 2;
        if hash1[i + half] - hash1[i] * b1.pow(half as u32) == hash_rev1[n - half] && hash2[i + half] - hash2[i] * b2.pow(half as u32) == hash_rev2[n - half] {
            print!("{}", std::str::from_utf8(&s).unwrap());
            let mut s = s;
            s[..i].reverse();
            println!("{}", std::str::from_utf8(&s[..i]).unwrap());
            return;
        }
    }
    unreachable!();
}
