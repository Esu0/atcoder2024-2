use proconio::input;

fn main() {
    input! {
        _: usize,
        s: proconio::marker::Bytes,
    }

    let mut s = s.iter().map(|&c| (1usize, c)).collect::<Vec<_>>();
    s.dedup_by(|a, b| if a.1 == b.1 {
        b.0 += 1;
        true
    } else { false });

    // eprintln!("{s:?}");
    let mut mx = [0; 26];
    for &(cnt, c) in &s {
        let i = (c - b'a') as usize;
        mx[i] = mx[i].max(cnt);
    }
    println!("{}", mx.iter().copied().sum::<usize>());
}
