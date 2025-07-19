use std::collections::BinaryHeap;

use proconio::input;

fn power_of_two_ranges(l: usize, r: usize, n: usize, mut f: impl FnMut(usize)) {
    power_of_two_ranges_rec(l, r, 0, n, 1, &mut f);
}

fn power_of_two_ranges_rec(l: usize, r: usize, a: usize, b: usize, i: usize, f: &mut impl FnMut(usize)) {
    if b <= l || r <= a {
        return;
    }
    if l <= a && b <= r {
        f(i);
        return;
    }
    let m = (a + b) / 2;
    power_of_two_ranges_rec(l, r, a, m, i * 2, f);
    power_of_two_ranges_rec(l, r, m, b, i * 2 + 1, f);
}

fn main() {
    input! {
        n: usize,
        m: usize,
        mut x: [u64; n],
    }

    let last = *x.last().unwrap();
    let n2 = n.next_power_of_two();
    while x.len() < n2 {
        x.push(last);
    }
    // let k = n2.trailing_zeros();
    let mut g = vec![vec![]; n2 * 5 + m * 2];

    let mut mxmn = vec![(u64::MAX, u64::MAX); n2];
    for i in 0..n2 {
        let j = (i + n2) / 2;
        if i % 2 == 0 {
            g[i].push((j + n2, x[i + 1] - x[i]));
            g[j + 3 * n2].push((i, x[i + 1] - x[i]));
            g[i].push((j + 2 * n2, 0));
            g[j + 4 * n2].push((i, 0));
            mxmn[j].1 = x[i];
        } else {
            g[i].push((j + n2, 0));
            g[j + 3 * n2].push((i, 0));
            g[i].push((j + n2 * 2, x[i] - x[i - 1]));
            g[j + n2 * 4].push((i, x[i] - x[i - 1]));
            mxmn[j].0 = x[i];
        }
    }

    for i in (2..n2).rev() {
        // eprintln!("{mxmn:?}");
        let j = i / 2;
        if i % 2 == 0 {
            g[i + n2].push((j + n2, mxmn[i + 1].0 - mxmn[i].0));
            g[j + n2 * 3].push((i + n2 * 3, mxmn[i + 1].0 - mxmn[i].0));
            g[i + 2 * n2].push((j + 2 * n2, 0));
            g[j + n2 * 4].push((i + 4 * n2, 0));
            mxmn[j].1 = mxmn[i].1;
        } else {
            g[i + n2].push((j + n2, 0));
            g[j + 3 * n2].push((i + 3 * n2, 0));
            g[i + 2 * n2].push((j + 2 * n2, mxmn[i].1 - mxmn[i - 1].1));
            g[j + 4 * n2].push((i + 4 * n2, mxmn[i].1 - mxmn[i - 1].1));
            mxmn[j].0 = mxmn[i].0;
        }
    }

    for i in 0..m {
        input! {
            l1: usize,
            r1: usize,
            l2: usize,
            r2: usize,
            c: u64,
        }

        if r1 < l2 {
            let xr1 = x[r1 - 1];
            power_of_two_ranges(l1 - 1, r1, n2, |j| {
                if j >= n2 {
                    g[j - n2].push((i + 5 * n2, xr1 - x[j - n2]));
                } else {
                    g[j + n2].push((i + 5 * n2, xr1 - mxmn[j].0));
                }
            });
            let xl2 = x[l2 - 1];
            power_of_two_ranges(l2 - 1, r2, n2, |j| {
                if j >= n2 {
                    g[i + 5 * n2 + m].push((j - n2, x[j - n2] - xl2));
                } else {
                    g[i + 5 * n2 + m].push((j + 4 * n2, mxmn[j].1 - xl2));
                }
            });

            g[i + 5 * n2].push((i + 5 * n2 + m, c + xl2 - xr1));
        } else {
            assert!(r2 < l1);
            let xl1 = x[l1 - 1];
            let xr2 = x[r2 - 1];
            power_of_two_ranges(l1 - 1, r1, n2, |j| {
                if j >= n2 {
                    g[j - n2].push((i + 5 * n2, x[j - n2] - xl1));
                } else {
                    g[j + 2 * n2].push((i + 5 * n2, mxmn[j].1 - xl1));
                }
            });
            power_of_two_ranges(l2 - 1, r2, n2, |j| {
                if j >= n2 {
                    g[i + 5 * n2 + m].push((j - n2, xr2 - x[j - n2]));
                } else {
                    g[i + 5 * n2 + m].push((j + 3 * n2, xr2 - mxmn[j].0));
                }
            });
            g[i + 5 * n2].push((i + 5 * n2 + m, c + xl1 - xr2));
        }
    }

    let mut dist = vec![u64::MAX; g.len()];
    let mut queue = BinaryHeap::from([(std::cmp::Reverse(0), 0)]);

    while let Some((std::cmp::Reverse(d), u)) = queue.pop() {
        if dist[u] != u64::MAX {
            continue;
        }
        dist[u] = d;
        for &(v, w) in &g[u] {
            if dist[v] == u64::MAX {
                queue.push((std::cmp::Reverse(d + w), v));
            }
        }
    }

    print!("{}", dist[1] as i64);
    for &dk in &dist[2..n] {
        print!(" {}", dk as i64);
    }
    println!();
}
