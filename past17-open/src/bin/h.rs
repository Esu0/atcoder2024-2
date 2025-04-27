use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(usize, usize, usize); n],
    }

    let mut classes = vec![vec![]; 5000];
    for &(ai, bi, ci) in &abc {
        classes[bi - 1].push((ai, ci));
    }

    let mut dp = vec![usize::MAX; m + 1];
    let mut dp_nxt = dp.clone();

    dp[0] = 0;
    for classes in &classes {
        dp_nxt.clone_from_slice(&dp);
        for &(ai, ci) in classes {
            for j in 0..=m {
                let dst = &mut dp_nxt[m.min(j + ci)];
                *dst = (*dst).min(dp[j].saturating_add(ai));
            }
        }
        dp.clone_from_slice(&dp_nxt);
    }

    println!("{}", dp[m] as isize);
}
