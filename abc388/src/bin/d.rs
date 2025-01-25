use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut queue = BinaryHeap::new();
    let mut ans = vec![usize::MAX; n];
    for i in 0..n {
        let ai = a[i] + queue.len();
        ans[i] = ai;
        queue.push(Reverse(ai + i));
        while queue.peek().is_some_and(|&Reverse(x)| x <= i) {
            queue.pop();
        }
    }
    for i in 0..n {
        ans[i] = ans[i].saturating_sub(n - i - 1);
    }
    print!("{}", ans[0]);
    for &ai in &ans[1..] {
        print!(" {}", ai);
    }
    println!();
}
