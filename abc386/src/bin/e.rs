use proconio::input;

fn combination(k: usize, i: usize, dat: &[usize], buf: &mut [usize], f: &mut impl FnMut(&[usize])) {
    if k == 0 {
        f(buf);
        return;
    }
    let n = dat.len();
    assert!(n >= k);
    for j in 0..=n - k {
        buf[i] = dat[j];
        combination(k - 1, i + 1, &dat[j + 1..], buf, f);
    }
}

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    if k >= n / 2 {
        let k = n - k;
        let xor_all = a.iter().copied().reduce(|acc, x| acc ^ x).unwrap();
        if k == 0 {
            println!("{xor_all}");
            return;
        }
        let mut buf = vec![0; k];
        let mut max = 0;
        combination(k, 0, &a, &mut buf, &mut |x| {
            max = max.max(x.iter().copied().reduce(|acc, x| acc ^ x).unwrap() ^ xor_all);
        });
        println!("{max}");
    } else {
        let mut buf = vec![0; k];
        let mut max = 0;
        combination(k, 0, &a, &mut buf, &mut |x| {
            max = max.max(x.iter().copied().reduce(|acc, x| acc ^ x).unwrap());
        });
        println!("{max}");
    }
}
