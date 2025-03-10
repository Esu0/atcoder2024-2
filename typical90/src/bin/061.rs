use proconio::input;

fn main() {
    input! {
        q: usize,
    }

    let mut deque = std::collections::VecDeque::new();

    for _ in 0..q {
        input! { t: u8 }
        if t == 1 {
            input! { x: u32 }
            deque.push_front(x);
        } else if t == 2 {
            input! { x: u32 }
            deque.push_back(x);
        } else if t == 3 {
            input! { x: usize }
            println!("{}", deque[x - 1]);
        }
    }
}
