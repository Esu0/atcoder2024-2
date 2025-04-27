use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }

    let mut cnt = vec![0usize; n];
    for &(ai, bi) in &ab {
        cnt[(ai + bi - 2) % n] += 1;
    }
    let mut ans = m * (m - 1) / 2;
    for &ci in &cnt {
        if ci > 0 {
            ans -= ci * (ci - 1) / 2;
        }
    }
    println!("{ans}");
}
