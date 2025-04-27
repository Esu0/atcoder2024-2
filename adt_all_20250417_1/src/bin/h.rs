use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut ans = vec![0; n];
    let mut queue1 = (0..n).map(std::cmp::Reverse).collect::<BinaryHeap<_>>();
    let mut queue2 = BinaryHeap::<(Reverse<u64>, usize)>::new();
    for _ in 0..m {
        input! {
            t: u64,
            w: u64,
            s: u64,
        }
        while let Some(&ev) = queue2.peek() {
            if ev.0.0 <= t {
                let p = queue2.pop().unwrap().1;
                queue1.push(Reverse(p));
            } else {
                break;
            }
        }

        if let Some(Reverse(p)) = queue1.pop() {
            ans[p] += w;
            queue2.push((Reverse(t + s), p));
        }
    }
    for &a in &ans {
        println!("{a}");
    }
}
