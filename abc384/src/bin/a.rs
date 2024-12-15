use proconio::{input, marker};

fn main() {
    input! {
        _n: usize,
        c1: char,
        c2: char,
        mut s: marker::Bytes,
    }

    for si in &mut s {
        if *si != c1 as u8 {
            *si = c2 as u8
        }
    }
    println!("{}", std::str::from_utf8(&s).unwrap());
}
