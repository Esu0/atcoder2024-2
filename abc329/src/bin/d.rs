use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut count = vec![0usize; n];
    let mut ans = (0, std::cmp::Reverse(usize::MAX));
    let mut s = String::with_capacity(m * 8);
    use std::fmt::Write;

    for _ in 0..m {
        input! { ai: usize }
        count[ai - 1] += 1;
        ans = ans.max((count[ai - 1], std::cmp::Reverse(ai)));
        // println!("{}", ans.1.0);
        writeln!(s, "{}", ans.1.0).unwrap();
    }
    print!("{s}");
}
