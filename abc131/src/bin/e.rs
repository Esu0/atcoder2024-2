use proconio::input;

// fn check(g: &[Vec<bool>], k: usize) -> bool {
//     let n = g.len();
//     for i in 0..n {
//         for j in i..n {
//             if g[i][j] != g[j][i] {
//                 return false;
//             }
//         }
//     }

//     let mut dp = g
//         .iter()
//         .map(|row| {
//             row.iter()
//                 .map(|&c| if c { 1 } else { u32::MAX })
//                 .collect::<Vec<_>>()
//         })
//         .collect::<Vec<_>>();
//     for i in 0..n {
//         dp[i][i] = 0;
//     }
//     for k in 0..n {
//         for i in 0..n {
//             for j in 0..n {
//                 dp[i][j] = dp[i][j].min(dp[i][k].saturating_add(dp[k][j]));
//             }
//         }
//     }

//     for i in 0..n {
//         if dp[i].iter().any(|&x| x == u32::MAX) {
//             return false;
//         }
//     }

//     let mut count = 0;
//     for i in 0..n {
//         count += dp[i].iter().filter(|&&x| x == 2).count();
//     }
//     eprintln!("{dp:?}");
//     count / 2 == k
// }

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let m = (n - 2) * (n - 1) / 2;
    if k > m {
        println!("-1");
        return;
    }

    let mut g = vec![vec![false; n]; n];
    for i in 1..n {
        g[0][i] = true;
        g[i][0] = true;
    }

    for (i, j) in (1..n)
        .flat_map(|i| (i + 1..n).map(move |j| (i, j)))
        .take(m - k)
    {
        g[i][j] = true;
        g[j][i] = true;
    }
    // assert!(check(&g, k));

    let mut ans = Vec::new();
    for i in 0..n {
        for j in i + 1..n {
            if g[i][j] {
                ans.push((i + 1, j + 1));
            }
        }
    }

    println!("{}", ans.len());
    for &(i, j) in &ans {
        println!("{} {}", i, j);
    }
}
