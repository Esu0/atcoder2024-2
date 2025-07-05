use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            abc: [(u64, u64, u64); n],
        }

        let (mut d1, mut d2) = (0, 0);
        let mut a = 0;
        let mut x = 0;
        let mut y = 0;
        for &(ai, bi, ci) in &abc {
            if ai + ci <= bi {
                d1 += ai;
                d2 += ci;
            } else {
                a += bi;
                x += bi - bi.min(ci);
                y += ai.min(bi);
            }
        }
        if d1 > d2 {
            (d1, d2) = (d2, d1);
            (x, y) = (a - y, a - x);
        }
        assert!(d1 <= d2);
        assert!(x <= y);
        let diff_min = (d2 + a - y) as i64 - (d1 + y) as i64;
        let diff_max = (d2 + a - x) as i64 - (d1 + x) as i64;
        assert_eq!(diff_min.rem_euclid(2), diff_max.rem_euclid(2));
        if (diff_min..=diff_max).contains(&0) {
            println!("{}", (a + d1 + d2) / 2);
        } else {
            println!("{}", (d2 + a - y).min(d1 + y).max((d2 + a - x).min(d1 + x)));
        }
    }
}
