use proconio::input;

fn main() {
    input! {
        n: usize,
        h: usize,
        m: usize,
        ab: [(usize, i32); n],
    }

    let mut dp = vec![i32::MIN; h + 1];
    let mut dp_nxt = dp.clone();
    dp[h] = m as i32;

    for (i, &(ai, bi)) in ab.iter().enumerate() {
        dp_nxt.fill(i32::MIN);
        for j in ai..=h {
            dp_nxt[j - ai] = dp_nxt[j - ai].max(dp[j]);
        }
        for j in 0..=h {
            if dp[j] >= bi {
                dp_nxt[j] = dp_nxt[j].max(dp[j] - bi);
            }
        }
        dp.clone_from_slice(&dp_nxt);
        if dp.iter().all(|&x| x < 0) {
            println!("{i}");
            return;
        }
    }
    println!("{n}");
}
