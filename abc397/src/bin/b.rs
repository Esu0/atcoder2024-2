use proconio::input;

fn main() {
    input! {
        s: proconio::marker::Bytes,
    }

    let mut ans = 0;
    let mut rem = true;
    for &si in &s {
        if rem && si == b'o' || !rem && si == b'i' {
            rem ^= true;
            ans += 1;
        }
        rem ^= true;
    }
    if *s.last().unwrap() == b'i' {
        ans += 1;
    }
    println!("{ans}");
}
