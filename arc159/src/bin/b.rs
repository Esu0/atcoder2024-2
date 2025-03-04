use proconio::input;

fn prime_factors(n: u64) -> Vec<(u64, usize)> {
    let mut n = n;
    let mut i = 2;
    let mut res = Vec::new();
    while i * i <= n {
        let mut count = 0;
        while n % i == 0 {
            n /= i;
            count += 1;
        }
        if count > 0 {
            res.push((i, count));
        }
        i += 1;
    }
    if n > 1 {
        res.push((n, 1));
    }
    res
}

fn gcd(a: u64, b: u64) -> u64 {
    let mut a = a;
    let mut b = b;
    while b > 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn main() {
    input! {
        mut a: u64,
        mut b: u64,
    }
    if b > a {
        (a, b) = (b, a);
    }

    let g = gcd(a, b);
    a /= g;
    b /= g;
    let mut factors = prime_factors(a - b);
    let mut buf = vec![];
    let mut ans = 0;
    while b > 0 {
        buf.clear();
        let mut min = b;
        for (i, &(pi, _)) in factors.iter().enumerate() {
            let rem = a % pi;
            use std::cmp::Ordering::*;
            match rem.cmp(&min) {
                Less => {
                    buf.clear();
                    buf.push(i);
                    min = rem;
                }
                Equal => {
                    buf.push(i);
                }
                _ => {}
            }
        }
        // eprintln!("{min}");
        // eprintln!("{buf:?}");
        a -= min;
        b -= min;
        ans += min;
        for &i in &buf {
            let pi = factors[i].0;
            let mut new_count = factors[i].1;
            // eprintln!("{a} {b} {pi}");
            while new_count > 0 && a % pi == 0 {
                a /= pi;
                assert_eq!(b % pi, 0);
                b /= pi;
                new_count -= 1;
            }
            factors[i].1 = new_count;
        }
        factors.retain(|&(_, c)| c != 0);
    }
    // eprintln!("{a} {b}");
    // eprintln!("{factors:?}");
    println!("{ans}");
}
