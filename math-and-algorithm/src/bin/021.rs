use proconio::input;

fn fact(n: u32) -> u128 {
    let mut res = 1;
    for i in 1..=n {
        res *= i as u128;
    }
    res
}

fn main() {
    input! {
        n: u32,
        r: u32,
    }

    println!("{}", fact(n) / fact(r) / fact(n - r));
}
