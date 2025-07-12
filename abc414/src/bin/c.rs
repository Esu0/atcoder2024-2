use proconio::input;

fn check_a(n: usize, a: u8) -> bool {
    let mut buf = [0; 100];
    let mut t = 0;
    let mut n = n;
    while n > 0 {
        buf[t] = (n % a as usize) as u8;
        n /= a as usize;
        t += 1;
    }

    for i in 0..t {
        if buf[i] != buf[t - i - 1] {
            return false;
        }
    }
    true
}

fn main() {
    input! {
        a: u8,
        n: usize,
    }

    let mut ans = 0usize;
    // let mut pp = 1usize;
    let mut p = 10;
    for k in 1..=6 {
        for i in 1..p {
            if i % 10 == 0 {
                continue;
            }
            let mut m = i;
            {
                let mut i = i;
                let mut p2 = p * p;
                for _ in 0..k {
                    p2 /= 10;
                    m += i % 10 * p2;
                    i /= 10;
                }
            }
            if check_a(m, a) && m <= n {
                // eprintln!("1: {m}");
                ans += m;
            }
            let mut m = i;
            // let orig_i = i;
            {
                let mut i = i;
                let mut p2 = p * p / 10;
                for _ in 0..k - 1 {
                    p2 /= 10;
                    m += i % 10 * p2;
                    i /= 10;
                }
                // if orig_i == 1 {
                //     eprintln!("{orig_i} {m}");
                // }
                if check_a(m, a) && m <= n {
                    // eprint!("{orig_i} ");
                    // eprintln!("{m}");
                    ans += m;
                }
            }
        }
        // for i in pp..p {
        // }
        // pp = p;
        p *= 10;
    }
    println!("{ans}");
}
