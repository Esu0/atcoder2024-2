use proconio::input;

fn solve(n: usize) {
    if n == 1 {
        print!("1 ");
    } else {
        solve(n - 1);
        print!("{n} ");
        solve(n - 1);
    }
}
fn main() {
    input! {
        n: usize,
    }
    solve(n);
    println!();
}
