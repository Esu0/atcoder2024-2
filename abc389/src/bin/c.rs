use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        q: usize,
    }

    let mut off = 0;
    let mut sum = 0;
    let mut queue = VecDeque::new();
    for _ in 0..q {
        input! { t: u8 }
        match t {
            1 => {
                input! { l: u64 }
                queue.push_back((l, sum));
                sum += l;
            }
            2 => {
                let head = queue.pop_front().unwrap();
                off += head.0;
            }
            3 => {
                input! { k: usize }
                let ans = queue[k - 1].1 - off;
                println!("{ans}");
            }
            _ => unreachable!()
        }
    }
}
