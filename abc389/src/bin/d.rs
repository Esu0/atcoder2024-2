use proconio::input;
use util::upper_bound;

fn main() {
    input! {
        r: i64
    }
    let dxy = [(1i64, 1i64), (1, -1), (-1, -1), (-1, 1)];
    if r < 500 {
        let mut count = 0u64;
        for x in -r..=r {
            for y in -r..=r {
                if dxy.iter().all(|&(dx, dy)| (2 * x + dx).pow(2) + (2 * y + dy).pow(2) <= 4 * r * r) {
                    count += 1;
                }
            }
        }
        println!("{count}");
        return;
    }
    let mut count = 0u64;
    for y in 1..r {
        let c = upper_bound(0..r + 1, |x| (2 * x + 1).pow(2) + (2 * y + 1).pow(2) < 4 * r * r);
        count += c as u64;
    }
    println!("{}", count * 4 + 1);
}
