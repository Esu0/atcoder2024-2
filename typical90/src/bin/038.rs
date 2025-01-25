use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64,
    }

    let gcd = {
        let mut a = a;
        let mut b = b;
        while b != 0 {
            (a, b) = (b, a % b);
        }
        a
    };
    let a = a / gcd;
    if let Some(ans) = a.checked_mul(b) {
        if ans <= 1_000_000_000_000_000_000 {
            println!("{ans}");
        } else {
            println!("Large");
        }
    } else {
        println!("Large");
    }
}
