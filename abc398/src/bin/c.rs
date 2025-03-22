use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }

    let mut set = BTreeMap::<u64, (usize, usize)>::new();
    for (i, &ai) in a.iter().enumerate() {
        set.entry(ai).and_modify(|c| c.0 += 1).or_insert((1, i));
    }

    for (&_, &v) in set.iter().rev() {
        if v.0 == 1 {
            println!("{}", v.1 + 1);
            return;
        }
    }
    println!("-1");
}
