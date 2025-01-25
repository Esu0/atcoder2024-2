use proconio::{input, marker};

fn main() {
    input! {
        _: usize,
        mut s: marker::Bytes,
        q: usize,
        txc: [(u8, usize, char); q],
    }

    let k = txc.iter().enumerate().rev().find(|&(_, &(t, _, _))| t == 2 || t == 3).map(|(i, _)| i);
    for (i, &(t, x, c)) in txc.iter().enumerate() {
        if t == 1 {
            s[x - 1] = c as u8;
        } else if Some(i) == k {
            for si in &mut s {
                *si = if t == 2 { si.to_ascii_lowercase() } else { si.to_ascii_uppercase() };
            }
        }
    }
    println!("{}", std::str::from_utf8(&s).unwrap());
}
