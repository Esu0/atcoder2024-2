use proconio::input;

fn main() {
    input! {
        s: proconio::marker::Bytes,
        t: proconio::marker::Bytes,
    }

    let n = s.len();
    if (1..n).filter(|&i| s[i].is_ascii_uppercase()).all(|i| t.contains(&s[i - 1])) {
        println!("Yes");
    } else {
        println!("No");
    }
}
