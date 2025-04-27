use proconio::input;

fn main() {
    input! {
        n: usize,
        d: u32,
        tl: [(u32, u32); n],
    }

    for k in 1..=d {
        let ans = tl.iter().map(|&(ti, li)| ti * (li + k)).max().unwrap();
        println!("{ans}");
    }
}
