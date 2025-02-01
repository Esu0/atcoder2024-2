use std::{cmp::Reverse, collections::{BinaryHeap, HashSet}};

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [u64; n],
        mut b: [u64; n],
        mut c: [u64; n],
    }

    a.sort_unstable_by_key(|&ai| Reverse(ai));
    b.sort_unstable_by_key(|&bi| Reverse(bi));
    c.sort_unstable_by_key(|&ci| Reverse(ci));

    let mut queue = BinaryHeap::from([(a[0] * b[0] + b[0] * c[0] + c[0] * a[0], 0, 0, 0)]);
    let mut visited = HashSet::from([(0, 0, 0)]);
    let mut count = 0;
    let f = |a: u64, b: u64, c: u64| {
        a * b + b * c + c * a
    };
    while let Some((v, i, j, k2)) = queue.pop() {
        count += 1;
        if k == count {
            println!("{v}");
            return;
        }
        if i + 1 < n && visited.insert((i + 1, j, k2)) {
            queue.push((f(a[i + 1], b[j], c[k2]), i + 1, j, k2));
        }
        if j + 1 < n && visited.insert((i, j + 1, k2)) {
            queue.push((f(a[i], b[j + 1], c[k2]), i, j + 1, k2));
        }
        if k2 + 1 < n && visited.insert((i, j, k2 + 1)) {
            queue.push((f(a[i], b[j], c[k2 + 1]), i, j, k2 + 1));
        }
    }
    unreachable!();
}
