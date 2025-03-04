use proconio::input;

fn main() {
    input! {
        n: usize,
        x: u32,
    }

    for _ in 0..n {
        input! { a: u32 }
        if a == x {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
