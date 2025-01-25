use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        d: usize,
    }

    let mut dp1 = vec![0.; l + d];
    dp1[0] = 1.;

    let mut csum = vec![0.; l + d + 1];
    csum[1] = 1.;
    for i in 1..l + d {
        dp1[i] = (csum[i.min(l)] - csum[i.saturating_sub(d)]) / d as f64;
        csum[i + 1] = csum[i] + dp1[i];
    }

    // eprintln!("{:?}", &dp1[l + 1..]);
    let burst = if l + d - 1 > n {
        *csum.last().unwrap() - csum[n + 1]
    } else {
        0.
    };
    let mut dp2 = vec![0.; n + d + 1];
    let mut csum2 = vec![0.; n + d + 2];
    for i in (0..=n).rev() {
        dp2[i] = (if i >= l + d {
            1.
        } else if i <= l {
            burst
        } else {
            csum[i] - csum[l] + burst
        })
        .max((csum2[i + 1] - csum2[i + 1 + d]) / d as f64);
        csum2[i] = csum2[i + 1] + dp2[i];
        // dp2[i] = ((sum1 - sum2) / d as f64).max()
    }
    // if csum.len() < 1000 {
    //     eprintln!("{csum:?}");
    //     eprintln!("{dp2:?}");
    // }
    println!("{}", dp2[0]);
}
