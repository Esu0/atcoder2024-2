use proconio::input;

fn solve(xs: &[i64]) -> i64 {
    let mut cnt = 0;
    let mut s0 = 0;
    let mut s1 = xs.iter().copied().sum::<i64>();
    let mut ans = i64::MAX;
    let n = xs.len() as i64;
    for &xi in xs {
        s1 -= xi;
        ans = ans.min(xi * cnt - s0 + s1 - xi * (n - cnt - 1));
        s0 += xi;
        cnt += 1;
    }
    ans
}

fn main() {
    input! { n: usize }
    let mut xs = Vec::with_capacity(n);
    let mut ys = Vec::with_capacity(n);

    for _ in 0..n {
        input! { xi: i64, yi: i64 }
        xs.push(xi);
        ys.push(yi);
    }
    xs.sort_unstable();
    ys.sort_unstable();
    println!("{}", solve(&xs) + solve(&ys));
}
