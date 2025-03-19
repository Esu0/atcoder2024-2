use proconio::input;
type MInt = util::ModInt<1000000007>;

fn main() {
    input! {
        l: u64,
        r: u64,
    }

    if l == 1_000_000_000_000_000_000 {
        assert_eq!(r, 1_000_000_000_000_000_000);
        println!("{}", MInt::new(19) * MInt::new(1_000_000_000_000_000_000));
        return;
    }

    let mut ans = MInt::new(0);
    let mut d1 = 0;
    {
        let mut l = l;
        while l > 0 {
            l /= 10;
            d1 += 1;
        }
    }
    let mut l = l;
    let mut d2 = 0;
    {
        let mut r = r;
        while r > 0 {
            r /= 10;
            d2 += 1;
        }
    }

    let mut p10 = 10u64.pow(d1);
    while d1 != d2 {
        ans += MInt::new((p10 - l) as _) * MInt::new((l + p10 - 1) as _) * MInt::new(d1 as _) / MInt::new(2);
        l = p10;
        d1 += 1;
        if d1 < d2 {
            p10 *= 10;
        }
    }
    ans += MInt::new((r - l + 1) as i64) * MInt::new((r + l) as _) * MInt::new(d1 as i64) / MInt::new(2);
    println!("{ans}");
}
