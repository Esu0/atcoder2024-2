use proconio::input;
use util::ModInt;
type MInt = ModInt<1000000007>;

// fn solve_naive(h: usize, w: usize, rc: &[(usize, usize)]) -> Vec<Vec<u32>> {
//     let mut is_wall = vec![vec![false; w]; h];
//     for &(ri, ci) in rc {
//         is_wall[ri - 1][ci - 1] = true;
//     }

//     let mut dp = vec![vec![MInt::new(0); w]; h];
//     dp[0][0] = MInt::new(1);
//     for i in 0..h {
//         for j in 0..w {
//             let dpij = dp[i][j];
//             if !is_wall[i][j] {
//                 if i + 1 < h {
//                     dp[i + 1][j] += dpij;
//                 }
//                 if j + 1 < w {
//                     dp[i][j + 1] += dpij;
//                 }
//             }
//         }
//     }
//     dp.iter().map(|dpi| dpi.iter().map(|&dpij| dpij.get()).collect::<Vec<_>>()).collect::<Vec<_>>()
// }

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        mut rc: [(usize, usize); n],
    }

    let mut fact = vec![MInt::new(1); h + w + 1];
    for i in 1..fact.len() {
        fact[i] = fact[i - 1] * MInt::new(i as _);
    }
    let fact_inv = fact.iter().map(|&x| x.inv()).collect::<Vec<_>>();
    rc.sort_unstable();
    let mut dp = vec![MInt::new(0); n];

    for (i, &(ri, ci)) in rc.iter().enumerate() {
        let wid = ci - 1;
        let hei = ri - 1;
        dp[i] = fact[wid + hei] * fact_inv[wid] * fact_inv[hei];
    }
    let mut ans = fact[h - 1 + w - 1] * fact_inv[h - 1] * fact_inv[w - 1];
    for (i, &(ri, ci)) in rc.iter().enumerate() {
        let dpi = dp[i];
        for (j, &(rj, cj)) in rc.iter().enumerate().skip(i + 1) {
            let Some(wid) = cj.checked_sub(ci) else { continue; };
            let Some(hei) = rj.checked_sub(ri) else { continue; };
            dp[j] -= dpi * fact[wid + hei] * fact_inv[wid] * fact_inv[hei];
        }
        let wid = w - ci;
        let hei = h - ri;
        ans -= dpi * fact[wid + hei] * fact_inv[wid] * fact_inv[hei];
    }
    println!("{ans}");
    // eprintln!("{rc:?}");
    // if h * w < 100000 {
    //     let mut map = vec![vec![0; w]; h];
    //     for (i, &(ri, ci)) in rc.iter().enumerate() {
    //         let ri = ri - 1;
    //         let ci = ci - 1;
    //         map[ri][ci] = dp[i].get();
    //     }
    //     let expected = solve_naive(h, w, &rc);
        // eprintln!("{expected:?}");
        // assert!(map.iter().enumerate().all(|(i, row)| row.iter().enumerate().all(|(j, &c)| c == 0 || c == expected[i][j])));
        // assert_eq!(expected[h - 1][w - 1], ans.get());
        // eprintln!("{:?}", map);
    // }
    
}
