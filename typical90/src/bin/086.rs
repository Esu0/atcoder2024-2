use proconio::input;
type MInt = util::ModInt<1000000007>;

fn main() {
    input! {
        n: usize,
        q: usize,
        xyzw: [(usize, usize, usize, u64); q],
    }

    let mut ans = MInt::new(1);

    for i in 0..60 {
        let mut cnt = 0;
        'nxt: for s in 0..1 << n {
            for &(xj, yj, zj, wj) in &xyzw {
                let xj = xj - 1;
                let yj = yj - 1;
                let zj = zj - 1;
                if ((s >> xj) | (s >> yj) | (s >> zj)) & 1 != (wj >> i) & 1 {
                    continue 'nxt;
                }
            }
            cnt += 1;
        }
        ans *= MInt::new(cnt);
    }
    println!("{ans}");
}
