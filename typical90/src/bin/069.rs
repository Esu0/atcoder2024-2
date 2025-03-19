use proconio::input;

type MInt = util::ModInt<1000000007>;

fn solve_simple(buf: &mut [u64], i: usize, k: u64, f: &mut impl FnMut(&[u64])) {
    let n = buf.len();
    if i == n {
        f(&*buf);
        return;
    }
    for j in 0..k {
        buf[i] = j;
        solve_simple(buf, i + 1, k, f);
    }
}

fn pow(base: MInt, exp: u64) -> MInt {
    let mut base = base;
    let mut exp = exp;
    let mut res = MInt::new(1);
    while exp > 0 {
        if exp & 1 != 0 {
            res *= base;
        }
        base = base * base;
        exp >>= 1;
    }
    res
}

fn main() {
    input! {
        n: u64,
        k: u64,
    }

    if n <= 10 && k <= 5 {
        let mut buf = vec![0; n as usize];
        let mut ans = 0u32;
        solve_simple(&mut buf, 0, k, &mut |cells| {
            for i in 0..n as usize - 1 {
                for j in i + 1..n as usize {
                    if i.abs_diff(j) <= 2 && cells[i] == cells[j] {
                        return;
                    }
                }
            }
            ans += 1;
        });
        println!("{ans}");
        return;
    }

    if n == 1 {
        println!("{}", k);
    } else {
        // n >= 2
        let k = MInt::new(k as _);
        println!("{}", k * (k - MInt::new(1)) * pow(k - MInt::new(2), n - 2));
    }
}
