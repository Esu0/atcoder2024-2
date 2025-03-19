use proconio::input;

fn main() {
    input! {
        n: usize,
        p: i64,
        k: usize,
        a: [[i64; n]; n],
    }

    let mut buf = vec![];
    let m = util::upper_bound(1..=p + 1, |x| {
        buf.clone_from(&a);
        for row in &mut buf {
            for cell in row {
                if *cell == -1 {
                    *cell = x;
                }
            }
        }
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    buf[i][j] = buf[i][j].min(buf[i][k] + buf[k][j]);
                }
            }
        }
        let mut cnt = 0;
        for i in 0..n - 1 {
            for j in i + 1..n {
                if buf[i][j] <= p {
                    cnt += 1;
                }
            }
        }
        cnt > k
    });

    if m == p + 2 {
        println!("0");
        return;
    }

    let m2 = util::upper_bound(1..=p + 1, |x| {
        buf.clone_from(&a);
        for row in &mut buf {
            for cell in row {
                if *cell == -1 {
                    *cell = x;
                }
            }
        }
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    buf[i][j] = buf[i][j].min(buf[i][k] + buf[k][j]);
                }
            }
        }
        let mut cnt = 0;
        for i in 0..n - 1 {
            for j in i + 1..n {
                if buf[i][j] <= p {
                    cnt += 1;
                }
            }
        }
        cnt >= k
    });

    // eprintln!("{m} {m2}");
    if m2 == p + 2 {
        println!("Infinity");
    } else {
        println!("{}", m2 - m);
    }
}
