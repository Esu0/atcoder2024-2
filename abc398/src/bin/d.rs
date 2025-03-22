use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut r: i32,
        mut c: i32,
        s: proconio::marker::Bytes,
    }

    let mut set = HashSet::from([(0, 0)]);

    let mut cur = (0, 0);
    let mut ans = Vec::with_capacity(n);
    for &si in &s {
        let diff = match si {
            b'N' => (1, 0),
            b'W' => (0, 1),
            b'S' => (-1, 0),
            b'E' => (0, -1),
            _ => unreachable!(),
        };
        r += diff.0;
        c += diff.1;
        cur.0 += diff.0;
        cur.1 += diff.1;
        set.insert(cur);
        ans.push(if set.contains(&(r, c)) { b'1' } else { b'0' });
    }
    println!("{}", std::str::from_utf8(&ans).unwrap());
}
