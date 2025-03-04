use proconio::input;

fn main() {
    input! {
        n: usize,
        x: u64,
        mut ud: [(u64, u64); n],
    }

    let mut ans = 0;
    for i in 0..n - 1 {
        if ud[i].0 > ud[i + 1].0 + x {
            let d = ud[i].0 - (ud[i + 1].0 + x);
            ans += d;
            ud[i].0 -= d;
        }
        if ud[i].0 + x < ud[i + 1].0 {
            let d = ud[i + 1].0 - (ud[i].0 + x);
            ans += d;
            ud[i + 1].0 -= d;
        }
        if ud[i].1 > ud[i + 1].1 + x {
            let d = ud[i].1 - (ud[i + 1].1 + x);
            ans += d;
            ud[i].1 -= d;
        }
        if ud[i].1 + x < ud[i + 1].1 {
            let d = ud[i + 1].1 - (ud[i].1 + x);
            ans += d;
            ud[i + 1].1 -= d;
        }
    }
    let hs = ud.iter().map(|&(ui, di)| ui + di).collect::<Vec<_>>();
    let min = hs.iter().copied().min().unwrap();
    ans += hs.iter().map(|&hi| hi - min).sum::<u64>();
    println!("{ans}");
}
