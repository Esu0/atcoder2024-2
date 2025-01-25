use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [u32; n],
    }
    let mut offset = 0;
    for _ in 0..q {
        input! { t: u8 }
        match t {
            1 => {
                input! { x: usize, y: usize }
                let x = x - 1;
                let y = y - 1;
                a.swap((x + offset) % n, (y + offset) % n);
            }
            2 => {
                input! { _: u8, _: u8 }
                offset = (offset + n - 1) % n;
            }
            3 => {
                input! { x: usize, _: u8 }
                let x = x - 1;
                println!("{}", a[(x + offset) % n]);
            }
            _ => unreachable!(),
        }
    }
}
