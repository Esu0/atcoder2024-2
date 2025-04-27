use proconio::input;

fn main() {
    input! {
        n: u128,
        m: u32,
    }

    let mut pn = 1;
    let mut s = 0;
    for _ in 0..=m {
        s += pn;
        if s > 1_000_000_000 {
            println!("inf");
            return;
        }
        pn *= n;
    }
    println!("{s}");
}
