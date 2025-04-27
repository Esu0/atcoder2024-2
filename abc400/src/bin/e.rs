use proconio::input;

fn main() {
    input! {
        q: usize,
    }

    let mut is_prime = [true; 1_000_001];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2usize..=1_000_000 {
        let mut j = i * 2;
        while j <= 1_000_000 {
            is_prime[j] = false;
            j += i;
        }
    }
    let primes = is_prime.iter().enumerate().filter(|x| *x.1).map(|(n, _)| n).collect::<Vec<_>>();
    // eprintln!("{:?}", &primes[..10]);
    let mut good = vec![];
    for (i, &n) in primes.iter().enumerate() {
        let n2 = n * n;
        let mut q = n2;
        while q <= 1_000_000_000_000 {
            for &m in &primes[i + 1..] {
                let m2 = m * m;
                let mut p = m2;
                let mut end = true;
                while q.checked_mul(p).is_some_and(|x| x <= 1_000_000_000_000) {
                    good.push(q * p);
                    end = false;
                    let Some(p_nxt) = p.checked_mul(m2) else {
                        break;
                    };
                    p = p_nxt;
                }
                if end {
                    break;
                }
            }
            let Some(q_nxt) = q.checked_mul(n2) else {
                break;
            };
            q = q_nxt;
        }
    }
    
    good.sort_unstable();

    for _ in 0..q {
        input! {
            a: usize,
        }

        let k = good.partition_point(|&x| x <= a);
        println!("{}", good[k - 1]);
    }
    // eprintln!("{good:?}");
}
