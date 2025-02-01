use std::cmp::Reverse;

use proconio::{input, marker};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: marker::Bytes,
        mut t: marker::Bytes,
    }
    if n == 1 {
        println!("{}", t[m - 1] - b'0');
        return;
    }
    let mut t2 = t.iter().copied().map(|x| x - b'0').enumerate().collect::<Vec<_>>();
    t2.sort_unstable_by_key(|&x| (Reverse(x.1), Reverse(x.0)));
    let mut ans = s.clone();
    let mut j = 0;
    for i in 0..n - 1 {
        let si = ans[i] - b'0';
        if j < m && si < t2[j].1 {
            ans[i] = t2[j].1 + b'0';
            t[t2[j].0] = 0;
            j += 1;
        }
    }
    // eprintln!("{:?}", t);
    if j < m {
        if *t.last().unwrap() == 0 || ans[..n - 1].contains(t.last().unwrap()) {
            ans[n - 1] = ans[n - 1].max(t2[j].1 + b'0');
        } else {
            ans[n - 1] = *t.last().unwrap();
        }
    }
    println!("{}", std::str::from_utf8(&ans).unwrap());
}
