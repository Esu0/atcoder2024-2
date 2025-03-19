use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
    }

    let mut diff = vec![0; n - 1];
    for i in 0..n - 1 {
        diff[i] = a[i + 1] - a[i];
    }

    let mut sum = diff.iter().map(|&di| di.abs()).sum::<i64>();

    for _ in 0..q {
        input! { l: usize, r: usize, v: i64 }
        let l = l - 1;
        if l > 0 {
            sum -= diff[l - 1].abs();
            diff[l - 1] += v;
            sum += diff[l - 1].abs();
        }
        if r < n {
            sum -= diff[r - 1].abs();
            diff[r - 1] -= v;
            sum += diff[r - 1].abs();
        }
        println!("{sum}");
    }
}
