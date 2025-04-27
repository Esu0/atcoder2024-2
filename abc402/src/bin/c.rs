use proconio::input;
use std::fmt::Write;
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [[usize]; m],
        b: [usize; n],
    }

    let mut rel = vec![vec![]; n];
    for (i, ai) in a.iter().enumerate() {
        for &aij in ai {
            rel[aij - 1].push(i);
        }
    }

    let mut cnts = a.iter().map(|ai| ai.len()).collect::<Vec<_>>();
    let mut ans = 0;
    let mut s = String::new();
    for &bi in &b {
        for &j in &rel[bi - 1] {
            cnts[j] -= 1;
            if cnts[j] == 0 {
                ans += 1;
            }
        }
        writeln!(&mut s, "{ans}").unwrap();
    }
    print!("{s}");
}
