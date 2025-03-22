use proconio::input;
use std::fmt::Write;
fn dfs(buf: &mut [usize], out: &mut String, cnt: &mut usize, mn: usize, i: usize, m: usize) {
    let n = buf.len();
    if i == n {
        *cnt += 1;
        write!(out, "{}", buf[0]).unwrap();
        for &bufi in &buf[1..] {
            write!(out, " {}", bufi).unwrap();
        }
        writeln!(out).unwrap();
        return;
    }
    let rem = n - i;

    for nxt in mn..=(m - (rem - 1) * 10) {
        buf[i] = nxt;
        dfs(buf, out, cnt, nxt + 10, i + 1, m);
    }
}
fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut buf = vec![usize::MAX; n];
    let mut out = String::new();
    let mut cnt = 0;
    dfs(&mut buf, &mut out, &mut cnt, 1, 0, m);
    println!("{cnt}");
    print!("{}", out);
}
