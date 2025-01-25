use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    }
    let mut sorted = a.clone();
    sorted.sort_unstable();
    let even_set = a.iter().copied().step_by(2).collect::<HashSet<_>>();
    let even_set_expected = sorted.iter().copied().step_by(2).collect::<HashSet<_>>();
    let mut count = 0;
    for &x in &even_set_expected {
        if !even_set.contains(&x) {
            count += 1;
        }
    }
    println!("{count}");
}
