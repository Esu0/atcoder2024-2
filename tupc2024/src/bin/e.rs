use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    let mut a = Vec::new();
    let mut stack = Vec::new();
    for _ in 0..t {
        input! {
            n: usize,
            s: proconio::marker::Bytes,
        }

        a.clear();
        let mut cnt0 = 0usize;
        let mut cnt02 = 0usize;
        for &si in &s {
            if si == b'1' {
                a.push(cnt0);
                cnt0 = 0;
            } else {
                assert_eq!(si, b'0');
                cnt02 += 1;
                cnt0 += 1;
            }
        }
        a.push(cnt0);

        // println!("{}:", std::str::from_utf8(&s).unwrap());
        if a.len() == 1 {
            println!("0");
            continue;
        }
        let cnt0 = a.iter().copied().sum::<usize>();
        assert_eq!(cnt02, cnt0);
        assert_eq!(a.len() - 1, n - cnt02);
        let mx = a.iter().copied().enumerate().max_by_key(|&(_, v)| v).unwrap();
        // eprintln!("{a:?}");
        if cnt0 == 0 {
            println!("{}", a.len() - 2);
            continue;
        }
        if mx.1 <= cnt0 / 2 {
            // if *a.first().unwrap() == 0 && *a.last().unwrap() == 0 {
            //     println!("{}", cnt0 / 2 + a.len() - 3);
            // } else {
            // }
            let mut ans = 0;
            stack.clear();
            stack.push(a[0]);
            for &ai in &a[1..] {
                let mut ai = ai;
                while let Some(prev) = stack.pop() {
                    if ai >= prev {
                        ans += prev;
                        ai -= prev;
                    } else {
                        ans += ai;
                        
                        break;
                    }
                }
            }
            println!("{}", cnt0 / 2 + a.len() - 2);
            continue;
        }
        if mx.0 == 0 || mx.0 == a.len() - 1 {
            println!("{}", cnt0 - mx.1 + a.len() - 2);
        } else {
            println!("{}", cnt0 - mx.1 + a.len() - 3);
        }
    }
}
