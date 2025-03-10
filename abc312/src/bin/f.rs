use std::cmp::Reverse;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut opened = Vec::with_capacity(n);
    let mut closed = Vec::with_capacity(n);
    let mut opener = Vec::with_capacity(n);

    for _ in 0..n {
        input! { t: u8, x: usize }
        if t == 0 {
            opened.push(x);
        } else if t == 1 {
            closed.push(x);
        } else {
            opener.push(x);
        }
    }

    opened.sort_unstable_by_key(|&x| Reverse(x));
    closed.sort_unstable_by_key(|&x| Reverse(x));
    opener.sort_unstable_by_key(|&x| Reverse(x));

    let mut cumsum = vec![0; opened.len() + 1];
    for i in 0..opened.len() {
        cumsum[i + 1] = cumsum[i] + opened[i];
    }

    let mut cumsum2 = vec![0; opener.len() + 1];
    for i in 0..opener.len() {
        cumsum2[i + 1] = cumsum2[i] + opener[i];
    }

    // let mut max_count = m;
    // let mut count = 0;
    let mut sum = 0;
    let mut ans = cumsum[opened.len().min(m)];
    // let mut openers = 0;

    for i in 0..=closed.len() {
        let x = cumsum2.partition_point(|&x| x < i);
        if x > opener.len() || i + x > m {
            break;
        }
        ans = ans.max(sum + cumsum[(m - i - x).min(opened.len())]);
        if i < closed.len() {
            sum += closed[i];
        }
    }
    println!("{ans}");
}
