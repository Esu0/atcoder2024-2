use proconio::input;

fn main() {
    input! {
        mut n: u64,
    }

    let mut fcount = n.trailing_zeros();
    n >>= fcount;
    let mut i = 3;
    while i * i <= n {
        while n % i == 0 {
            fcount += 1;
            n /= i;
        }
        i += 2;
    }
    assert_ne!(n, 0);
    if n != 1 {
        fcount += 1;
    }

    for i in 0.. {
        if fcount <= (1 << i) {
            println!("{i}");
            return;
        }
    }
}
