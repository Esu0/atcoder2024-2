use proconio::input;
use util::algorithm;

fn main() {
    input! {
        n: usize,
        c: u64,
        h: [u64; n],
    }

    // let mut dp = vec![u64::MAX; n];
    // let mut ind = vec![usize::MAX; n];
    // let mut buf = vec![i64::MAX; n];
    // dp[0] = 0;
    // for i in 1..n {
    //     for j in 0..i {
    //         buf[j] = (dp[j] + (h[i] - h[j]).pow(2)) as _;
    //         dp[i] = dp[i].min(dp[j] + (h[i] - h[j]).pow(2) + c);
    //     }
    //     ind[i] = buf.iter().enumerate().min_by_key(|&(_, &x)| x).unwrap().0;
    //     eprintln!("{buf:?}");
    //     for j in 1..i {
    //         buf[j - 1] -= buf[j];
    //     }
    //     eprintln!("{buf:?}");
    //     eprintln!("{dp:?}");
    // }
    // eprintln!("{dp:?}");
    // eprintln!("{ind:?}");
    // println!("{}", dp[n - 1]);

    // let mut dp = vec![u64::MAX; n];
    // dp[0] = 0;
    // for i in 0..n {
    //     for j in i + 1..n {
    //         dp[j] = dp[j].min(dp[i] + (h[j] - h[i]).pow(2) + c);
    //     }
    //     eprintln!("{dp:?}");
    // }
    // println!("{}", dp[n - 1]);

    // curve equation: (x - curve.2)^2 + curve.1
    let mut curve_pos = 0;
    let mut dp_curves = vec![(0, c, h[0])];
    for (i, &hi) in h.iter().enumerate().skip(1) {
        while curve_pos + 1 < dp_curves.len() && dp_curves[curve_pos + 1].0 <= i {
            curve_pos += 1;
        }
        let curve = dp_curves[curve_pos];
        let dpi = (hi - curve.2).pow(2) + curve.1;
        let new_curve = (dpi + c, hi);
        let new_curve_pos1 = curve_pos + 1 + dp_curves[curve_pos + 1..].partition_point(|&curve| {
            (h[curve.0] - curve.2).pow(2) + curve.1 <= (h[curve.0] - new_curve.1).pow(2) + new_curve.0
        });
        assert_ne!(new_curve_pos1, 0);
        dp_curves.truncate(new_curve_pos1);
        let curve = dp_curves[new_curve_pos1 - 1];
        let new_curve_pos2 = algorithm::upper_bound(curve.0.max(i)..n, |j| {
            if j >= n {
                return false;
            }
            // eprintln!("{i} {j}");
            (h[j] - curve.2).pow(2) + curve.1 <= (h[j] - new_curve.1).pow(2) + new_curve.0
        });
        if new_curve_pos2 != n {
            dp_curves.push((new_curve_pos2, new_curve.0, new_curve.1));
        }
        // eprintln!("{dp_curves:?}");
    }

    let curve = *dp_curves.last().unwrap();
    let ans = (h[n - 1] - curve.2).pow(2) + curve.1;
    // assert_eq!(dp[n - 1], ans);
    println!("{ans}");

}
