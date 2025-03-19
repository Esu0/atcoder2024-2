use proconio::input;

fn digsum(mut n: usize) -> usize {
    let mut res = 0;
    for _ in 0..5 {
        res += n % 10;
        n /= 10;
    }
    res
}

fn op(buf: &mut [usize], a: &[usize], b: &[usize]) {
    for (i, di) in buf.iter_mut().enumerate() {
        *di = b[a[i]];
    }
}

fn main() {
    input! {
        n: usize,
        k: u64,
    }

    let mut res = (0..100000).collect::<Vec<_>>();
    let mut base = (0..100000)
        .map(|x| (digsum(x) + x) % 100000)
        .collect::<Vec<_>>();
    let mut k = k;
    let mut buf1 = vec![0; 100000];
    let mut buf2 = vec![0; 100000];
    while k > 0 {
        if k & 1 != 0 {
            buf1.clone_from_slice(&res);
            op(&mut res, &buf1, &base);
        }
        buf1.clone_from_slice(&base);
        buf2.clone_from_slice(&base);
        op(&mut base, &buf1, &buf2);
        k >>= 1;
    }
    println!("{}", res[n]);
}
