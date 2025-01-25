use proconio::{input, marker};

fn main() {
    input! {
        s: marker::Bytes,
    }

    let mut ans = 0;
    let mut i = 0;
    let n = s.len();
    while i < n {
        if i + 1 < n && &s[i..i + 2] == b"00" {
            ans += 1;
            i += 2;
        } else {
            ans += 1;
            i += 1;
        }
    }
    println!("{ans}");
}
