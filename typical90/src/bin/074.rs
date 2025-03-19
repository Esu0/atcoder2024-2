use proconio::input;

fn main() {
    input! {
        _: usize,
        s: proconio::marker::Bytes,
    }

    let mut ans = 0;
    for (i, &si) in s.iter().enumerate() {
        ans += ((si - b'a') as u64) << i;
    }
    println!("{ans}");
}
