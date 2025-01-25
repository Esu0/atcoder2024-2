use proconio::input;

fn main() {
    input! {
        n: usize,
        d: u32,
        mut tl: [(u32, u32); n],
    }

    for _ in 1..=d {
        for (_, li) in tl.iter_mut() {
            *li += 1;
        }
        let ans = tl.iter().map(|&(ti, li)| ti * li).max().unwrap();
        println!("{ans}");
    }
}
