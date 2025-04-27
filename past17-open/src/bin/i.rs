use std::collections::{HashMap, HashSet};

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [proconio::marker::Bytes; n],
    }

    let mut cnt = HashMap::<&[u8], usize>::new();
    for si in &s {
        *cnt.entry(&si[..]).or_default() += 1;
    }

    let mut ans = 0;
    for si in &s {
        let mut set = HashSet::new();
        for t in 1..1 << si.len() {
            let mut ps = Vec::with_capacity(5);
            for i in 0..si.len() {
                if (t >> i) & 1 != 0 {
                    ps.push(si[i]);
                }
            }
            set.insert(ps);
        }
        for ps in &set {
            ans += cnt.get(&ps[..]).copied().unwrap_or_default();
        }
    }
    println!("{ans}");
}
