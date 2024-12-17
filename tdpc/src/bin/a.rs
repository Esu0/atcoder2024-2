use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }

    let mut dp = vec![false; 10001];
    let mut dp_next = vec![];
    dp[0] = true;
    for &pi in &p {
        dp_next.clone_from(&dp);
        for j in 0..=10000 {
            let nj = j + pi;
            if nj <= 10000 && dp[j] {
                dp_next[nj] = true;
            }
        }
        dp.clone_from(&dp_next);
    }
    let ans = dp.iter().filter(|x| **x).count();
    println!("{ans}");
}
