use proconio::input;
use segtree::operation;
fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut exists = (0..n)
        .map(|_| 1u32)
        .collect::<segtree::Segtree<_, operation::Add<_>>>();
    let mut seg = (0..n)
        .map(|_| 0i64)
        .collect::<segtree::Segtree<_, operation::Add<_>>>();

    for _ in 0..q {
        input! { t: u8, x: usize, y: usize, v: i64 }
        let x = x - 1;
        let y = y - 1;
        if t == 0 {
            exists.update(x, 0);
            if x % 2 == 0 {
                seg.update(x, v);
            } else {
                seg.update(x, -v);
            }
        } else if x < y {
            if exists.query(x..y) != 0 {
                println!("Ambiguous");
                continue;
            }
            let mut q = seg.query(x..y);
            if y % 2 == 0 {
                q = -q;
            }
            let ans = if (y - x) % 2 == 1 { q - v } else { q + v };
            println!("{ans}");
        } else if y == x {
            println!("{v}");
        } else {
            if exists.query(y..x) != 0 {
                println!("Ambiguous");
                continue;
            }
            let mut q = seg.query(y..x);
            if y % 2 == 1 {
                q = -q;
            }
            let ans = if (x - y) % 2 == 1 { q - v } else { q + v };
            println!("{ans}");
        }
    }
}
