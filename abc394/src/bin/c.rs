use proconio::input;

fn main() {
    input! {
        mut s: proconio::marker::Bytes,
    }

    let mut w_cnt = 0;
    for i in 0..s.len() {
        if s[i] == b'A' && w_cnt > 0 {
            s[i - w_cnt] = b'A';
            s[i - w_cnt + 1..=i].fill(b'C');
        }
        if s[i] == b'W' {
            w_cnt += 1;
        } else {
            w_cnt = 0;
        }
    }
    println!("{}", std::str::from_utf8(&s).unwrap());
}
