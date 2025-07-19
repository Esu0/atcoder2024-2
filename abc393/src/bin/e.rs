use std::io::BufWriter;

use proconio::input;
const M: usize = 1_000_000;
fn main() {
    input! {
        n: usize,
        k: u32,
        a: [usize; n],
    }
    let mut is_prime = vec![true; M + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut cnt = vec![0u32; M + 1];
    for &ai in &a {
        cnt[ai] += 1;
    }

    for p in 2..=M {
        if !is_prime[p] {
            continue;
        }

        let mut j = p * 2;
        while j <= M {
            is_prime[j] = false;
            j += p;
        }

        let mut b = M / p;
        while b > 0 {
            cnt[b] += cnt[b * p];
            b -= 1;
        }
    }
    // eprintln!("{:?}", &cnt[..100]);

    let mut dp = cnt.iter().enumerate().map(|(i, &ci)| if ci >= k { i as u32 } else { 0 }).collect::<Vec<_>>();
    for (p, _) in is_prime.iter().enumerate().filter(|&(_, &c)| c) {
        let mut b = 1;
        while b * p <= M {
            dp[b * p] = dp[b * p].max(dp[b]);
            b += 1;
        }
    }

    let mut writer = BufWriter::new(std::io::stdout());
    use std::io::Write;

    for &ai in &a {
        writeln!(writer, "{}", dp[ai]).unwrap();
    }
}
