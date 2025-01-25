use proconio::{input, marker};

fn digsum(n: u32) -> u32 {
    let mut sum = 0;
    let mut n = n;
    while n > 0 {
        sum += n % 10;
        n /= 10;
    }
    sum
}

fn main() {
    input! {
        s: marker::Bytes,
    }
    if s.len() <= 5 {
        // 1 ~ 99999
        let mut n = 0;
        for &c in s.iter() {
            n = n * 10 + (c - b'0') as u32;
        }
        for i in n..n * 2 {
            if i % digsum(i) == 0 && (i + 1) % digsum(i + 1) == 0 {
                println!("{i}");
                return;
            }
        }
        println!("-1");
    } else {
        let n = ((s[0] - b'0') * 10 + (s[1] - b'0')) as u32;
        let ans = match n {
            10..=16 => 17,
            17..=25 => 26,
            26..=34 => 35,
            35..=43 => 44,
            44..=52 => 53,
            53..=61 => 62,
            62..=70 => 71,
            71..=79 => 80,
            80..=99 => 110,
            _ => unreachable!(),
        };
        println!("{ans}{}", "0".repeat(s.len() - 2));
    }
}
