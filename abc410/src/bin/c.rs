use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut a = (1..=n).collect::<Vec<_>>();
    let mut rot = 0;
    for _ in 0..q {
        input! { t: u8 }
        if t == 3 {
            input! { k: usize }
            rot = (rot + k) % n;
        } else if t == 2 {
            input! { x: usize }
            println!("{}", a[(x - 1 + rot) % n]);
        } else {
            input! { x: usize, y: usize }
            a[(x - 1 + rot) % n] = y;
        }
    }
}
