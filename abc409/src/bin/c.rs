fn main() {
    proconio::input! {
        n: usize,
        l: usize,
        d: [usize; n - 1],
    }

    if l % 3 != 0 {
        println!("0");
        return;
    }
    let mut count = vec![0usize; l];
    let mut prev = 0;
    count[0] = 1;
    for &di in &d {
        let t = (prev + di) % l;
        count[t] += 1;
        prev = t;
    }

    let m = l / 3;
    let mut ans = 0;
    for i in 0..m {
        ans += count[i] * count[i + m] * count[i + m * 2];
    }
    println!("{ans}");
}
