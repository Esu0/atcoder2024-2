use proconio::input;

fn main() {
    input! {
        n: usize,
        m: u64,
        mut a: [[u64; n]; n],
    }
    if n == 1 {
        println!("{}", a[0][0] % m);
        return;
    }
    let mut pow10 = vec![1; 2 * n + 2];
    for i in 0..pow10.len() - 1 {
        pow10[i + 1] = pow10[i] * 10 % m;
    }

    for (i, ai) in a.iter_mut().enumerate() {
        for (j, aij) in ai.iter_mut().enumerate() {
            *aij = *aij * pow10[n - i - 1 + n - j - 1] % m;
        }
    }

    let mut half0 = vec![vec![]; n];
    let mut half1 = vec![vec![]; n];
    for p in 0..1 << (n - 1) {
        let mut row = 0;
        let mut pos = (0, 0);
        let mut val = 0;
        for i in 0..n - 1 {
            val = (val + a[pos.0][pos.1]) % m;
            if (p >> i) & 1 == 0 {
                row += 1;
                pos.0 += 1;
            } else {
                pos.1 += 1;
            }
        }
        half0[row].push((val + a[pos.0][pos.1]) % m);
    }
    for p in 0..1 << (n - 1) {
        let mut row = n - 1;
        let mut pos = (n - 1, n - 1);
        let mut val = 0;
        for i in 0..n - 1 {
            val = (val + a[pos.0][pos.1]) % m;
            if (p >> i) & 1 == 0 {
                row -= 1;
                pos.0 -= 1;
            } else {
                pos.1 -= 1;
            }
        }
        half1[row].push(val);
    }

    for v in &mut half0 {
        v.sort_unstable();
    }

    let mut ans = 0;
    for (v0, v1) in half0.iter().zip(&half1) {
        for &x in v1 {
            let tmp = v0.partition_point(|&v0i| v0i + x < m);
            if tmp > 0 {
                ans = ans.max(v0[tmp - 1] + x);
            }
            ans = ans.max((v0.last().unwrap() + x) % m);
        }
    }
    println!("{ans}");
}
