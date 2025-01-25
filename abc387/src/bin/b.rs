use proconio::input;

fn main() {
    input! {
        x: u32,
    }

    let mut ans = 0;
    for i in 1..=9 {
        for j in 1..=9 {
            let p = i * j;
            if p != x {
                ans += p;
            }
        }
    }
    println!("{ans}");
}
