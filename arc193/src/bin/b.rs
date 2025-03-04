use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: proconio::marker::Bytes,
    }

    let mut pairs = (0..n).map(|i| (i, (i + 1) % n)).collect::<Vec<_>>();
    for (i, &si) in s.iter().enumerate() {
        if si == b'1' {
            pairs.push((i, n));
        }
    }

    if pairs.len() > 16 {
        panic!();
    }
    let mut set = HashMap::<Vec<u8>, usize>::new();
    for s in 0..1usize << pairs.len() {
        let mut t = vec![0u8; n + 1];
        for i in 0..pairs.len() {
            if s & (1 << i) != 0 {
                t[pairs[i].1] += 1;
            } else {
                t[pairs[i].0] += 1;
            }
        }
        set.entry(t).and_modify(|c| *c += 1).or_insert(1);
    }
    eprintln!("{set:?}");
    println!("{}", set.len());
}
