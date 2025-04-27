use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        cv: [(usize, u64); n],
    }

    let mut dp = vec![[(usize::MAX, 0); 2]; k + 1];
    let mut dp_nxt = dp.clone();
    dp[0][0].0 = 0;
    for &(ci, vi) in &cv {
        dp_nxt[1..].clone_from_slice(&dp[..k]);
        dp_nxt[0] = [(usize::MAX, 0); 2];
        for (&dpj, dst) in dp.iter().zip(&mut dp_nxt) {
            let mx = if dpj[0].0 == ci {
                dpj[1]
            } else {
                dpj[0]
            };
            if mx.0 == usize::MAX {
                continue;
            }
            let s = mx.1 + vi;
            let mut end = false;
            for k in 0..2 {
                if dst[k].0 == ci {
                    end = true;
                    if s > dst[k].1 {
                        dst[k].1 = s;
                    }
                }
            }
            if end {
                if dst[0].1 < dst[1].1 {
                    dst.swap(0, 1);
                }
                continue;
            }
            if s > dst[0].1 {
                dst[1] = dst[0];
                dst[0] = (ci, s);
            } else if s > dst[1].1 {
                dst[1] = (ci, s);
            }
        }
        dp.clone_from_slice(&dp_nxt);
        // eprintln!("{dp:?}");
    }
    if dp[k][0].0 == usize::MAX {
        println!("-1");
    } else {
        println!("{}", dp[k][0].1);
    }
}
