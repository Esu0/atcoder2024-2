use proconio::input;

fn main() {
    input! {
        s: proconio::marker::Bytes,
    }

    let n = s.len();
    let mut ans = 0;
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                if i <= j && j <= k && k - j == j - i && s[i] == b'A' && s[j] == b'B' && s[k] == b'C' {
                    ans += 1;
                }
            }
        }
    }
    println!("{ans}");
}
