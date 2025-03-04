fn main() {
    proconio::input! {
        n: usize,
        lr: [(usize, usize); n],
    }
    let mut set = vec![false; 1 << n];
    for s in 0..1u32 << n {
        let mut k = 0usize;
        for i in 0..n {
            if s & (1 << i) != 0 {
                let (li, ri) = lr[i];
                k ^= ((1 << ri) - 1) ^ ((1 << (li - 1)) - 1);
            }
        }
        set[k] = true;
    }
    let ok = !set.contains(&false);
    println!("{ok}");
}
