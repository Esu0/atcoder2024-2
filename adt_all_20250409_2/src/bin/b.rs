use proconio::input;

fn main() {
    input! {
        mut n: u32,
        m: u32,
        x: u32,
        mut t: u32,
        d: u32,
    }

    while n != m {
        if n <= x {
            t -= d;
        }
        n -= 1;
    }

    println!("{t}");
}
