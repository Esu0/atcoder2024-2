use proconio::input;

fn main() {
    input! {
        n: usize,
        c: [usize; n],
        x: [u64; n],
    }
    let mut dp = vec![vec![u64::MAX; n]; n + 1];
    dp[0].fill(0);
    let mut buf1 = vec![0; n];
    let mut prev = vec![0; n];
    let mut first = vec![0; n];
    for d in 1..=n {
        for l in 0..n {
            prev.fill(usize::MAX);
            first.fill(usize::MAX);
            for i in 0..d {
                let ci = c[(l + i) % n];
                buf1[i] = prev[ci - 1];
                if prev[ci - 1] == usize::MAX {
                    
                }
                prev[ci - 1] = i;
            }
        }
    }
}
