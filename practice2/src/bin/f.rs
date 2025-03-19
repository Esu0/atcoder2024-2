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
        n: usize,
        m: usize,
        a: [u32; n],
        b: [u32; m],
    }

    let mut a = a.iter().map(|&ai| MInt::new(ai as _)).collect::<Vec<_>>();
    let mut b = b.iter().map(|&bi| MInt::new(bi as _)).collect::<Vec<_>>();
    let sz = (n + m - 1).next_power_of_two();
    a.resize(sz, MInt::default());
    b.resize(sz, MInt::default());
    convolution_inplace(&mut a, &mut b);

    print!("{}", a[0]);
    for &ci in &a[1..n + m - 1] {
        print!(" {}", ci);
    }
    println!();
}
