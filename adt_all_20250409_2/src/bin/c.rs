use proconio::input;

fn main() {
    input! {
        n: usize,
        s: proconio::marker::Bytes,
    }

    let mut ans = vec![];
    let mut i = 0;
    while i < n {
        if i + 1 < n && &s[i..i + 2] == b"na" {
            ans.extend(*b"nya");
            i += 2;
        } else {
            ans.push(s[i]);
            i += 1;
        }
    }

    println!("{}", std::str::from_utf8(&ans).unwrap());
}
