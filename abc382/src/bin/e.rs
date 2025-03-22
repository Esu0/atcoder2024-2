use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        p: [u32; n],
    }

    let mut dp0 = vec![0.; n + 1];
    let mut dp0_nxt = dp0.clone();
    dp0[0] = 1.;
    for (i, &pi) in p.iter().enumerate() {
        dp0_nxt.fill(0.);
        let pin = (100 - pi) as f64 / 100.;
        let pi = pi as f64 / 100.;
        for j in 0..=i {
            dp0_nxt[j] += dp0[j] * pin;
            dp0_nxt[j + 1] += dp0[j] * pi;
        }
        dp0.clone_from_slice(&dp0_nxt);
    }

    let mut dp1 = vec![0.; x + n + 1];
    for i in (0..x).rev() {
        let mut s = 1.;
        for j in 1..=n {
            s += dp1[i + j] * dp0[j];
        }
        dp1[i] = s / (1. - dp0[0]);
    }
    println!("{}", dp1[0]);
}
