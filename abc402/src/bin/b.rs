use proconio::input;

fn main() {
    input! {
        q: usize,
    }

    let mut queue = std::collections::VecDeque::new();
    for _ in 0..q {
        input! { t: u8 }
        if t == 1 {
            input! { x: i64 }
            queue.push_back(x);
        } else {
            println!("{}", queue.pop_front().unwrap());
        }
    }
}
