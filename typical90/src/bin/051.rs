use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        p: u64,
        a: [u64; n],
    }

    if n <= 20 {
        let mut ans = 0u64;
        for s in 0u32..1 << n {
            let popcount = s.count_ones();
            if popcount != k as u32 {
                continue;
            }
            let mut sum = 0;
            for i in 0..n {
                if s & (1 << i) != 0 {
                    sum += a[i];
                }
            }
            if sum <= p {
                ans += 1;
            }
        }
        println!("{ans}");
        return;
    }

    let half = n / 2;
    let popcount = (0..1u32 << (n - half))
        .map(|s| s.count_ones() as usize)
        .collect::<Vec<_>>();
    let mut sums1 = vec![vec![]; k + 1];
    for s in 0..1usize << half {
        let mut sum = 0;
        for i in 0..half {
            if s & (1 << i) != 0 {
                sum += a[i];
            }
        }
        let p = popcount[s];
        if p <= k {
            sums1[popcount[s]].push(sum);
        }
    }
    sums1.iter_mut().for_each(|row| row.sort_unstable());

    let mut ans = 0;
    for t in 0..1usize << (n - half) {
        let mut sum = 0;
        for i in half..n {
            if t & (1 << (i - half)) != 0 {
                sum += a[i];
            }
        }
        if sum > p {
            continue;
        }
        let popc = popcount[t];
        if popc <= k {
            let p2 = k - popc;
            ans += sums1[p2].partition_point(|&x| x + sum <= p);
        }
    }
    println!("{ans}");
}
