use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        q: usize,
    }

    let mut queue = VecDeque::new();
    for _ in 0..q {
        input! { t: u8 }
        if t == 1 {
            input! {
                c: u64,
                x: u64,
            }
            queue.push_back((x, c));
        } else {
            input! { mut k: u64 }
            let mut sum = 0;
            while queue.front().unwrap().1 < k {
                let (x, c) = queue.pop_front().unwrap();
                sum += x * c;
                k -= c;
            }
            sum += queue.front().unwrap().0 * k;
            queue.front_mut().unwrap().1 -= k;
            if queue.front().unwrap().1 == 0 {
                queue.pop_front().unwrap();
            }
            println!("{sum}");
        }
    }
}
