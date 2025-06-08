use proconio::input;

fn main() {
    input! {
        s: proconio::marker::Bytes,
    }

    let mut ans = 0;
    let mut prev = 0;
    for &si in s.iter().rev() {
        let d = si - b'0';
        ans += ((d + 10 - prev) % 10) as usize;
        prev = d;
    }
    println!("{}", ans + s.len());
}
