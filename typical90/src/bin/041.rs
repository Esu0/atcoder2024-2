use std::cmp::Reverse;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut xy: [(i64, i64); n],
    }
    let mut slopes = Vec::<(i64, i64)>::new();
    let conv1 = {
        xy.sort_unstable();
        let mut conv1 = Vec::<(i64, i64)>::with_capacity(n);
        for &(xi, yi) in &xy {
            while let Some(&(x0, y0)) = conv1.last() {
                let prev_slope = *slopes.last().unwrap();
                if prev_slope.0 * (xi - x0) > (yi - y0) * prev_slope.1 {
                    break;
                } else {
                    conv1.pop();
                    slopes.pop();
                }
            }
            if let Some(&(x0, y0)) = conv1.last() {
                slopes.push((yi - y0, xi - x0));
            } else {
                slopes.push((1, 0));
            }
            // eprintln!("{slopes:?}");
            conv1.push((xi, yi));
        }
        conv1
    };

    let conv2 = {
        let mut conv2 = Vec::<(i64, i64)>::with_capacity(n);
        slopes.clear();
        xy.sort_unstable_by_key(|&(xi, yi)| (xi, Reverse(yi)));
        for &(xi, yi) in &xy {
            while let Some(&(x0, y0)) = conv2.last() {
                let prev_slope = *slopes.last().unwrap();
                if prev_slope.0 * (xi - x0) < (yi - y0) * prev_slope.1 {
                    break;
                } else {
                    conv2.pop();
                    slopes.pop();
                }
            }
            slopes.push(if let Some(&(x0, y0)) = conv2.last() {
                (yi - y0, xi - x0)
            } else {
                (-1, 0)
            });
            conv2.push((xi, yi));
        }
        conv2
    };
}
