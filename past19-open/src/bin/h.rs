use proconio::input;

fn permutation(buf: &mut [u64], i: usize, f: &mut impl FnMut(&[u64])) {
    let n = buf.len();
    
    if i == n {
        f(&*buf);
    }
    for j in i..n {
        buf.swap(i, j);
        permutation(buf, i + 1, f);
        buf.swap(i, j);
    }
}

fn main() {
    input! {
        n: usize,
        s: u64,
        a: [u64; n],
    }

    let mut a = a;
    let mut buf = vec![];
    let mut ans = None;
    permutation(&mut a, 0, &mut |p| {
        if ans.is_some() {
            return;
        }
        for t in 0u32..1 << (n - 1) {
            buf.clear();
            buf.extend(p.iter().copied());
            // eprintln!("{buf:?} {t}");
            for i in 0..n - 1 {
                if t & (1 << i) != 0 {
                    buf[i + 1] *= buf[i];
                    buf[i] = 0;
                }
                // eprintln!("{buf:?} {t}");
            }
            let a = buf.iter().copied().sum::<u64>();
            if s == a {
                ans = Some((p.to_vec(), t));
            }
        }
    });
    if let Some(ans) = ans {
        println!("Yes");
        print!("{}", ans.0[0]);
        for i in 1..n {
            let op = if ans.1 & (1 << (i - 1)) != 0 {
                'x'
            } else {
                '+'
            };
            print!("{op}{}", ans.0[i]);
        }
        println!();
    } else {
        println!("No");
    }
}
