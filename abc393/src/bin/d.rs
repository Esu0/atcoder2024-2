use proconio::input;

fn main() {
    input! {
        n: usize,
        s: proconio::marker::Bytes,
    }

    let mut count1 = 0;
    let mut ans = 0;
    for (i, &si) in s.iter().enumerate() {
        if si == b'1' {
            ans += i - count1;
            count1 += 1;
        }
    }

    let mut diff = -(count1 as i64);
    let mut cur = ans as i64;
    let mut ans = cur;
    for &si in &s[..n - count1] {
        
    }
}
