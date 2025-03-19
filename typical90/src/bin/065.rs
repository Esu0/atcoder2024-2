use proconio::input;
type MInt = util::ModInt<998244353>;

const P: MInt = MInt::new(3);
// W23^(2^23) = 1 (mod 998244353)
const W23: MInt = P.pow(7 * 17);
const W23_INV: MInt = W23.inv();

const fn get_ws(w: MInt) -> [MInt; 24] {
    let mut buf = [MInt::new(0); 24];
    let mut i = 23;
    buf[i] = w;
    while i > 0 {
        buf[i - 1] = buf[i].mul_const(buf[i]);
        i -= 1;
    }
    buf
}

const fn get_ninv() -> [MInt; 24] {
    let mut buf = [MInt::new(1); 24];
    let base = MInt::new(2).inv();
    let mut i = 1;
    while i < 24 {
        buf[i] = buf[i - 1].mul_const(base);
        i += 1;
    }
    buf
}

// WS[i]^(2^i) = 1 (mod 998244353)
const WS: [MInt; 24] = get_ws(W23);
const WS_INV: [MInt; 24] = get_ws(W23_INV);
const N_INV: [MInt; 24] = get_ninv();

fn fft(arr: &mut [MInt]) {
    let n = arr.len();
    assert!(n.is_power_of_two());

    let mut i = n.trailing_zeros();
    while i > 0 {
        let w0 = WS[i as usize];
        i -= 1;
        let half = 1 << i;
        let mut offset = 0;
        while offset < n {
            let mut w = MInt::new(1);
            for j in 0..half {
                let a = arr[j + offset];
                let b = arr[j + offset + half];
                arr[j + offset] = a + b;
                arr[j + offset + half] = (a - b) * w;
                w *= w0;
            }
            offset += half * 2;
        }
    }
}

fn ifft(arr: &mut [MInt]) {
    let n = arr.len();
    assert!(n.is_power_of_two());

    let m = n.trailing_zeros();
    for i in 0..m {
        let w0 = WS_INV[(i + 1) as usize];
        let half = 1 << i;
        let mut offset = 0;
        while offset < n {
            let mut w = MInt::new(1);
            for j in 0..half {
                let a = arr[j + offset];
                let b = arr[j + offset + half] * w;
                arr[j + offset] = a + b;
                arr[j + offset + half] = a - b;
                w *= w0;
            }
            offset += half * 2;
        }
    }
}

// a <- a * b
fn convolution_inplace(a: &mut [MInt], b: &mut [MInt]) {
    assert_eq!(a.len(), b.len());
    fft(a);
    fft(b);
    for (ai, &bi) in a.iter_mut().zip(&*b) {
        *ai *= bi;
    }
    ifft(a);
    let m = a.len().trailing_zeros() as usize;
    for ai in a {
        *ai *= N_INV[m];
    }
}

fn main() {
    input! {
        r: usize,
        g: usize,
        b: usize,
        k: usize,
        x: usize,
        y: usize,
        z: usize,
    }

    let mut fact = vec![MInt::new(1); b.max(g).max(r) + 1];
    for i in 1..fact.len() {
        fact[i] = fact[i - 1] * MInt::new(i as _);
    }
    let ifact = fact.iter().map(|&x| x.inv()).collect::<Vec<_>>();
    let sz = (g + b + 1).next_power_of_two();
    let mut a1 = vec![MInt::new(0); sz];
    let mut a2 = a1.clone();
    if k - z > x {
        println!("0");
        return;
    }
    for (i, a1i) in a1[..=x.min(g)].iter_mut().enumerate().skip(k - z) {
        *a1i = fact[g] * ifact[i] * ifact[g - i];
    }
    for (i, a2i) in a2[..=z.min(b)].iter_mut().enumerate().skip(k - x) {
        *a2i = fact[b] * ifact[i] * ifact[b - i];
    }

    convolution_inplace(&mut a1, &mut a2);

    // eprintln!("{:?}", a1);
    let mut ans = MInt::new(0);
    for i in 0..=r.min(k) {
        let j = k - i;
        if j <= g + b && j <= y {
            ans += a1[j] * fact[r] * ifact[i] * ifact[r - i];
        }
    }
    // if r + g + b <= 21 {
    //     let mut expected = 0;
    //     for s in 0u32..1 << r {
    //         for t in 0u32..1 << g {
    //             for u in 0u32..1 << b {
    //                 let ps = s.count_ones();
    //                 let pt = t.count_ones();
    //                 let pu = u.count_ones();
    //                 if ps + pt + pu != k as u32 {
    //                     continue;
    //                 }
    //                 if ps + pt <= x as u32 && pt + pu <= y as u32 && pu + ps <= z as u32 {
    //                     expected += 1;
    //                 }
    //             }
    //         }
    //     }
    //     assert_eq!(ans.get(), expected);
    // }
    println!("{ans}");
}
