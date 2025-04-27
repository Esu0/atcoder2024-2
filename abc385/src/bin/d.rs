use std::collections::{BTreeSet, HashMap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        sx: i64,
        sy: i64,
        xy: [(i64, i64); n],
    }

    let mut ys = HashMap::new();
    let mut xs = HashMap::new();
    for &(xi, yi) in &xy {
        ys.entry(xi).or_insert_with(BTreeSet::new).insert(yi);
        xs.entry(yi).or_insert_with(BTreeSet::new).insert(xi);
    }

    let (mut x, mut y) = (sx, sy);
    let mut cnt = 0;
    let mut buf = vec![];
    for _ in 0..m {
        input! { d: char, c: i64 }
        match d {
            'U' => {
                if let Some(ys) = ys.get_mut(&x) {
                    for &yi in ys.range(y + 1..=y + c) {
                        buf.push(yi);
                    }
                    while let Some(yi) = buf.pop() {
                        ys.remove(&yi);
                        xs.get_mut(&yi).unwrap().remove(&x);
                        cnt += 1;
                    }
                }
                y += c;
            }
            'D' => {
                if let Some(ys) = ys.get_mut(&x) {
                    for &yi in ys.range(y - c..y) {
                        buf.push(yi);
                    }
                    while let Some(yi) = buf.pop() {
                        ys.remove(&yi);
                        xs.get_mut(&yi).unwrap().remove(&x);
                        cnt += 1;
                    }
                }
                y -= c;
            }
            'R' => {
                if let Some(xs) = xs.get_mut(&y) {
                    for &xi in xs.range(x + 1..=x + c) {
                        buf.push(xi);
                    }
                    while let Some(xi) = buf.pop() {
                        xs.remove(&xi);
                        ys.get_mut(&xi).unwrap().remove(&y);
                        cnt += 1;
                    }
                }
                x += c;
            }
            'L' => {
                if let Some(xs) = xs.get_mut(&y) {
                    for &xi in xs.range(x - c..x) {
                        buf.push(xi);
                    }
                    while let Some(xi) = buf.pop() {
                        xs.remove(&xi);
                        ys.get_mut(&xi).unwrap().remove(&y);
                        cnt += 1;
                    }
                }
                x -= c;
            }
            _ => unreachable!(),
        }
    }
    println!("{x} {y} {cnt}");
}
