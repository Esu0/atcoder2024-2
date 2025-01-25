use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        vac: [(u8, i64, usize); n],
    }

    let mut ac = [vec![], vec![], vec![]];
    for &(vi, ai, ci) in &vac {
        ac[(vi - 1) as usize].push((ai, ci));
    }

    let mut dps = [vec![], vec![], vec![]];
    for (i, ac) in ac.iter().enumerate() {
        dps[i] = vec![vec![i64::MIN; x + 1]; ac.len() + 1];
        dps[i][0][0] = 0;
        for (j, &(aj, cj)) in ac.iter().enumerate() {
            for k in 0..=x {
                dps[i][j + 1][k] = dps[i][j + 1][k].max(dps[i][j][k]);
                if k + cj <= x {
                    dps[i][j + 1][k + cj] = dps[i][j + 1][k + cj].max(dps[i][j][k] + aj);
                }
            }
        }
        for j in 0..x {
            dps[i][ac.len()][j + 1] = dps[i][ac.len()][j + 1].max(dps[i][ac.len()][j]);
        }
    }
    // eprintln!("{:?}", dps[0][ac[0].len()]);
    // eprintln!("{:?}", dps[1][ac[1].len()]);
    // eprintln!("{:?}", dps[2][ac[2].len()]);

    let mut mx = 0;
    for i in 0..=x {
        for j in 0..=x {
            if i + j > x {
                continue;
            }
            let k = x - i - j;
            mx = mx.max(dps[0][ac[0].len()][i].min(dps[1][ac[1].len()][j]).min(dps[2][ac[2].len()][k]));
        }
    }
    println!("{mx}");
}
