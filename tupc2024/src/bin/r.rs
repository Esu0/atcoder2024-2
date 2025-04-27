use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    // let mut score = Vec::new();
    'next: for _ in 0..t {
        input! { mut s: proconio::marker::Bytes }

        let qcnt = s.iter().filter(|&&c| c == b'?').count();
        if qcnt == 0 {
            let mut score = 0;
            for &si in s.iter() {
                if si == b'(' {
                    score += 1;
                } else if si == b')' {
                    score -= 1;
                }
                if score < 0 {
                    println!("Second");
                    continue 'next;
                }
            }
            if score == 0 {
                println!("First");
            } else {
                println!("Second");
            }
            continue;
        }
        if qcnt % 2 == 0 {
            println!("Second");
            continue;
        }

        let mut score = 0i32;
        for &si in s.iter() {
            if si == b'(' {
                score += 1;
            } else if si == b')' {
                score -= 1;
            }
        }
        if score == -1 {
            s.reverse();
            for si in &mut s {
                if *si == b'(' {
                    *si = b')';
                } else if *si == b')' {
                    *si = b'(';
                }
            }
            score = 1;
        }

        if score != 1 {
            println!("Second");
            continue;
        }

        let mut qcnt = 0;
        score = 0;
        for &si in &s {
            if si == b'(' {
                score += 1;
            } else if si == b')' {
                score -= 1;
            } else {
                qcnt += 1;
            }
            if qcnt % 2 == 1 && score <= 0 {
                println!("Second");
                continue 'next;
            }
            if qcnt % 2 == 0 && score <= -1 {
                println!("Second");
                continue 'next;
            }
        }
        println!("First");
    }
}
