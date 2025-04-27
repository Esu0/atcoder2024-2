use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! { s: u64, t: u64 }

        {
            let mut x = ((s + t) as f64).sqrt() as u64 - 1;
            while x * x < s + t {
                x += 1;
            }
            if x * x == s + t {
                println!("1 {t}");
                continue;
            }
        }

        let diff = s.abs_diff(t);
        if diff != 1 && diff != 4 {
            if diff % 2 == 1 {
                let a = (diff - 1) / 2;
                let b = (diff + 1) / 2;
                if s > t {
                    if b * b > s {
                        assert_eq!((b * b - s) + t, a * a);
                        println!("2 {} {}", b * b - s, t);
                        continue;
                    }
                } else if a * a > s {
                    assert_eq!((a * a - s) + t, b * b);
                    println!("2 {} {}", a * a - s, t);
                    continue;
                }
            }
            if diff % 4 == 0 {
                let a = (diff / 2 - 2) / 2;
                let b = (diff / 2 + 2) / 2;
                if s > t {
                    if b * b > s {
                        assert_eq!((b * b - s) + t, a * a);
                        println!("2 {} {}", b * b - s, t);
                        continue;
                    }
                } else if a * a > s {
                    assert_eq!((a * a - s) + t, b * b);
                    println!("2 {} {}", a * a - s, t);
                    continue;
                }
            }
        }

        let mut a = (((s + 1500000000) as f64).sqrt() as u64) / 2 * 2;
        if s % 2 == t % 2 {
            a += 1;
        }

        let u = a * a - s;
        print!("3 {} ", u);
        assert!(u > t);
        let d = u - t;
        assert_eq!(d % 2, 1);
        let a = (d - 1) / 2;
        let b = (d + 1) / 2;
        let v = b * b - u;
        assert_eq!(a * a - v, t);
        println!("{v} {}", t);
    }
}
