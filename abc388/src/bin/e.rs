use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }

    let f = |k| {
        if k * 2 > n {
            return false;
        }
        let mut ok = true;
        for i in 0..k {
            if a[i] * 2 > a[n - k + i] {
                ok = false;
                break;
            }
        }
        ok
    };
    let (mut l, mut r) = (0, n / 2 + 1);
    while r - l >= 2 {
        let m = l + (r - l) / 2;
        if f(m) {
            l = m + 1;
        } else {
            r = m;
        }
    }
    let ans = if l == r {
        l
    } else if f(l) {
        l + 1
    } else {
        l
    };

    // let ans = upper_bound(0..=n / 2 + 1, |k| {
    //     let mut ok = true;
    //     for i in 0..k {
    //         if a[i] * 2 > a[n - k + i] {
    //             ok = false;
    //             break;
    //         }
    //     }
    //     ok
    // });
    // println!("{}", ans - 1);
    println!("{}", ans - 1);
}
