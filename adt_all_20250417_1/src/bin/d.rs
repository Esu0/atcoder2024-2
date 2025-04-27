use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut l = 0;
    let mut r = 1;
    let mut ans = 0;
    for _ in 0..q {
        input! {
            h: char,
            t: usize,
        }
        let mut diff = usize::MAX;
        if h == 'L' {
            let mut d = 0;
            {
                let mut l = l;
                while l != t - 1 {
                    l = (l + 1) % n;
                    d += 1;
                    if l == r {
                        d = usize::MAX;
                        break;
                    }
                }
            }
            diff = diff.min(d);
            d = 0;
            {
                let mut l = l;
                while l != t - 1 {
                    l = (l + n - 1) % n;
                    d += 1;
                    if l == r {
                        d = usize::MAX;
                        break;
                    }
                }
                diff = diff.min(d);
                ans += diff;
            }
            l = t - 1;
        } else {
            let mut d = 0;
            {
                let mut r = r;
                while r != t - 1 {
                    r = (r + 1) % n;
                    d += 1;
                    if l == r {
                        d = usize::MAX;
                        break;
                    }
                }
            }
            diff = diff.min(d);
            d = 0;
            {
                let mut r = r;
                while r != t - 1 {
                    r = (r + n - 1) % n;
                    d += 1;
                    if l == r {
                        d = usize::MAX;
                        break;
                    }
                }
                diff = diff.min(d);
                ans += diff;
            }
            r = t - 1;
        }
    }
    println!("{ans}");
}
