use std::cmp::Reverse;

use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
        c: u32,
        d: u32,
        e: u32,
    }
    let points = [a, b, c, d, e];
    let mut strings = std::array::from_fn::<_, 32, _>(|i| {
        let mut string = Vec::with_capacity(5);
        for j in 0..5 {
            if (i & (1 << j)) != 0 {
                string.push(b'A' + j);
            }
        }
        string
    });
    strings[1..].sort_unstable();
    strings[1..].sort_by_key(|s| {
        let mut sum = 0;
        for &si in s {
            sum += points[(si - b'A') as usize];
        }
        Reverse(sum)
    });
    for s in &strings[1..] {
        println!("{}", std::str::from_utf8(s).unwrap());
    }
}
