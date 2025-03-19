use proconio::input;

fn main() {
    input! {
        n: u64,
    }

    for d in 1u64..=1_000_000 {
        let y = util::upper_bound(1u64..=1_000_000_000, |y| {
            let y = y as u128;
            let d = d as u128;
            3 * d * y * y + 3 * d * d * y + d * d * d < n as u128
        });
        let x = (y + d) as u128;
        let y = y as u128;
        if x * x * x - y * y * y == n as u128 {
            println!("{} {}", x, y);
            return;
        }
    }
    println!("-1");
}
