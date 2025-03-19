use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        xy: [(u64, u64); n],
    }

    let mut dist = vec![vec![0; n]; n];
    for (i, &(xi, yi)) in xy.iter().enumerate() {
        for (j, &(xj, yj)) in xy.iter().enumerate() {
            dist[i][j] = xi.abs_diff(xj).pow(2) + yi.abs_diff(yj).pow(2);
        }
    }
    let boxed = (0..=k).map(|_| u64::MAX).collect::<Box<[_]>>();
    let mut dp = vec![boxed; 1 << n];
    dp[0][0] = 0;
    for i in 0..n {
        dp[1 << i][1] = 0;
    }
    let mut indice = (1usize..1 << n)
        .map(|i| (i, i.count_ones()))
        .collect::<Vec<_>>();
    indice.sort_unstable_by_key(|x| x.1);

    let mut dists = vec![0; 1 << n];
    for i in 0..n {
        for s in 0..1 << i {
            let mut mx = dists[s];
            for j in 0..i {
                if (s >> j) & 1 != 0 {
                    mx = mx.max(dist[i][j]);
                }
            }
            dists[s | (1 << i)] = mx;
        }
    }
    // eprintln!("{dists:?}");
    for &(s, _) in &indice {
        let mut t = s;
        while t > 0 {
            t = (t - 1) & s;
            let u = s ^ t;
            for i in 1..=k {
                dp[s][i] = dp[s][i].min(dp[t][i - 1].max(dists[u]));
            }
        }
    }
    println!("{}", dp[(1 << n) - 1][k]);
}
