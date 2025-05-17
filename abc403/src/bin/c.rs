use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        _m: usize,
        q: usize,
    }
    let mut s = HashSet::new();
    let mut all = vec![false; n];
    for _ in 0..q {
        input! {
            t: u8,
        }

        if t == 1 {
            input! { x: usize, y: usize }
            s.insert((x - 1, y - 1));
        } else if t == 2 {
            input! { x: usize }
            all[x - 1] = true;
        } else {
            input! { x: usize, y: usize }
            if all[x - 1] || s.contains(&(x - 1, y - 1)) {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}
