use std::collections::HashSet;

use proconio::input;
use rand::Rng;

fn main() {
    input! {
        n: usize,
        mut s: proconio::marker::Bytes,
    }

    if s[0] == b'B' {
        for si in &mut s {
            if *si == b'A' {
                *si = b'B';
            } else {
                *si = b'A';
            }
        }
    }

    let mut cumsum = vec![0; n];
    for (i, &si) in s[1..].iter().enumerate() {
        cumsum[i] = cumsum[i - 1] + if si == b'A' { 1i32 } else { -1 };
    }

    use std::cmp::Ordering::*;

    let m1 = 998244353u64;
    let m2 = 1000000007u64;
    let mut rng = rand::thread_rng();
    let b1 = rng.gen_range(2..m1 - 1);
    let mut rev = 0;
    let mut pb = vec![1; n];
    for (i, &ci) in cumsum[1..].iter().enumerate() {
        let c = match ci.cmp(&(-1)) {
            Greater => 1,
            Equal => 3,
            Less => 2,
        };
        rev = (rev * b1 + c) % m1;
        pb[i + 1] = pb[i] * b1 % m1;
    }

    let mut set = HashSet::from([rev + pb]);
    for &ci in &cumsum[1..] {
        let c = match ci.cmp(&(-1)) {
            Greater => 1,
            Equal => 3,
            Less => 2,
        };
    }
}
