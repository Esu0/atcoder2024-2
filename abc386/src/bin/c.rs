use proconio::{input, marker};

fn main() {
    input! {
        _: u8,
        mut s: marker::Bytes,
        mut t: marker::Bytes,
    }

    if s == t {
        println!("Yes");
        return;
    }

    if s.len() < t.len() {
        std::mem::swap(&mut s, &mut t);
    }

    if s.len() == t.len() {
        let x = (0..s.len()).filter(|&i| s[i] != t[i]).count();
        if x == 1 {
            println!("Yes");
            return;
        }
    } else if s.len() == t.len() + 1 {
        let mut j = 0;
        for i in 0..s.len() {
            if j < t.len() && s[i] == t[j] {
                j += 1;
            }
        }
        if s.len() - j <= 1 {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
