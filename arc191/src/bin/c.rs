use proconio::input;

const fn is_prime(n: u64) -> bool {
    const AS: [u64; 7] = [
        2,
        325,
        9375,
        28178,
        450775,
        9780504,
        1795265022,
    ];

    if n == 2 {
        return true;
    }
    if n < 2 || n % 2 == 0 {
        return false;
    }

    let r = (n - 1).trailing_zeros();
    let d = (n - 1) >> r;

    let mut i = 0;
    'outer: while i < AS.len() {
        let ai = AS[i] % n;
        if ai == 0 {
            i += 1;
            continue;
        }
        let mut apow = 1u128;
        let n = n as u128;
        {
            let mut base = ai as u128;
            let mut exp = d;
            while exp > 0 {
                if exp & 1 != 0 {
                    apow = apow * base % n;
                }
                base = base * base % n;
                exp >>= 1;
            }
        }
        if apow == 1 || apow == n - 1 {
            i += 1;
            continue;
        }
        let mut j = 0;
        while j < r - 1 {
            apow = apow * apow % n;
            if apow == n - 1 {
                i += 1;
                continue 'outer;
            }
            j += 1;
        }
        return false;
    }
    true
}

fn mpow(base: u64, mut exp: u64, m: u64) -> u64 {
    let mut base = base as u128;
    let m = m as u128;
    let mut res = 1;
    while exp > 0 {
        if exp & 1 != 0 {
            res = res * base % m;
        }
        base = base * base % m;
        exp >>= 1;
    }
    res as _
}

fn main() {
    input! {
        t: usize,
        ns: [u64; t],
    }
    let primes = {
        let mut is_prime = vec![true; 40000];
        is_prime[0] = false;
        is_prime[1] = false;
        for i in 2..40000 {
            if is_prime[i] {
                let mut j = i * 2;
                while j < 40000 {
                    is_prime[j] = false;
                    j += i;
                }
            }
        }
        is_prime.iter().copied().enumerate().filter(|&(_, b)| b).map(|x| x.0 as u32).collect::<Vec<_>>()
    };
    // eprintln!("{primes:?}");
    // primes.iter().for_each(|&x| assert!(is_prime(x as u64)));
    let mut factors = vec![];
    let mut rng = rand::thread_rng();
    for &n in &ns {
        if n == 1 {
            println!("1 1");
            continue;
        }
        let mut m = 0;
        let mut k = 0;
        for i in 1..10000 {
            m = n * i + 1;
            k = i;
            if is_prime(m) {
                break;
            }
        }
        if m == 0 { panic!() }
        factors.clear();
        {
            let mut n = m - 1;
            let c2 = n.trailing_zeros();
            if c2 != 0 {
                factors.push((2, c2));
            }
            for &p in &primes {
                let p = p as u64;
                let mut count = 0;
                while n % p == 0 {
                    n /= p;
                    count += 1;
                }
                if count != 0 {
                    factors.push((p as u32, count));
                }
            }
            if n > 1 {
                factors.push((n as u32, 1));
            }
        }
        use rand::Rng;
        let g = loop {
            let g = rng.gen_range(1..m);
            if factors.iter().all(|&(pi, _)| mpow(g, (m - 1) / pi as u64, m) != 1) {
                break g;
            }
        };
        let a = mpow(g, k, m);

        // let a = g;

        // check
        // {
        //     let a = a as u128;
        //     let m = m as u128;
        //     let mut x = 1u128;
        //     for _ in 0..n - 1 {
        //         x = x * a % m;
        //         assert_ne!(x, 1, "n = {n}, m = {m}, a = {a}");
        //     }
        //     x = x * a % m;
        //     assert_eq!(x, 1, "n = {n}, m = {m}, a = {a}");
        // }
        println!("{a} {m}");
    }
}
