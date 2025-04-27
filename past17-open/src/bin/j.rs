use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(u32, u32); n],
        q: usize,
    }

    let mut imos = Vec::with_capacity(n * 2 + 1);
    imos.push((0, 0));
    for &(ai, bi) in &ab {
        imos.push((ai, 1));
        imos.push((bi + 1, -1));
    }
    imos.sort_unstable_by_key(|x| x.0);
    for i in 0..imos.len() - 1 {
        imos[i + 1].1 += imos[i].1;
    }

    use std::fmt::Write;
    let mut s = String::new();
    for _ in 0..q {
        input! { t: u32 }
        let k = imos.partition_point(|&(time, _)| time <= t);
        writeln!(&mut s, "{}", imos[k - 1].1).unwrap();
    }
    print!("{s}");
}
