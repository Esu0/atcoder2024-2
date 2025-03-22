use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut map = HashMap::new();
    for i in 1..=n {
        input! { ai: u32 }
        if let Some(prev) = map.insert(ai, i) {
            print!("{} ", prev);
        } else {
            print!("-1 ");
        }
    }
    println!();
}
