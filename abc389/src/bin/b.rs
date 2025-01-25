use proconio::input;

fn main() {
    input! {
        x: u64,
    }

    let mut fact = 1u64;
    for n in 2..100 {
        fact *= n;
        if fact == x {
            println!("{}", n);
            return;
        }
    }
    unreachable!()
}
