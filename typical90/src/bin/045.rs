use proconio::input;

fn solve(k: usize, dist: &[Vec<u64>], threshold: u64, buf: &mut [(u64, u64)]) -> bool {
    let n = dist.len();
    buf.fill((0, 0));
    buf[0] = (1, 1);
    let m1 = 2013265921;
    let m2 = 1811939329;
    for i in 0..n {
        for b in 0..1 << i {
            let b2 = b | (1 << i);
            buf[b2].0 = buf[b].0;
            let mut b3 = 0;
            for j in 0..i {
                if b & (1 << j) != 0 && dist[i][j] <= threshold {
                    b3 |= 1 << j;
                }
            }
            buf[b2].0 += buf[b3].0;
        }
    }
    for i in 0..1 << n {
        let t = buf[i].0 % m1;
        let mut r1 = 1;
        let mut r2 = 1;
        for _ in 0..k {
            r1 = r1 * t % m1;
            r2 = r2 * t % m2;
        }
        buf[i] = (r1, r2);
    }

    // buf.iter().enumerate().for_each(|(i, &c)| {
    //     eprintln!("{:02$b} {}", i, c, n)
    // });
    // eprintln!();
    for i in 0..n {
        for j in 0..1 << n {
            if j & (1 << i) != 0 {
                buf[j].0 = (buf[j].0 + m1 - buf[j & !(1 << i)].0) % m1;
                buf[j].1 = (buf[j].1 + m2 - buf[j & !(1 << i)].1) % m2;
            }
        }
    }

    // buf.iter().enumerate().for_each(|(i, &c)| {
    //     eprintln!("{:02$b} {}", i, c, n)
    // });

    buf[(1 << n) - 1] == (0, 0)
    // for i in 0..n {
    //     for j in 0..1 << i {
    //         if buf[j] == 0 {
    //             continue;
    //         }
    //         let mut ok = true;
    //         for k in 0..i {
    //             if j & (1 << k) != 0 && dist[i][k] > threshold {
    //                 ok = false;
    //                 break;
    //             }
    //         }
    //         buf[j | (1 << i)] = ok as u8;
    //     }
    // }

    // buf.iter_mut().for_each(|bi| if *bi == 0 { *bi = u8::MAX });
    // for i in 1..1 << n {
    //     let mut j = (i - 1) & i;
    //     while j > 0 {
    //         buf[i] = buf[i].min(buf[j].saturating_add(buf[i & !j]));
    //         j = (j - 1) & i;
    //     }
    // }
    // buf[(1 << n) - 1] as usize
}

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
    // eprintln!("{dist:?}");
    let mut dp = vec![(0, 0); 1 << n];
    // solve(k, &dist, 1, &mut dp);
    let ans = util::upper_bound(1u64.., |t| solve(k, &dist, t, &mut dp));
    println!("{}", ans);
}
