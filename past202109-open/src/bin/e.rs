use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let mut cp = vec![(0, 0); n];
    for (ci, _) in &mut cp {
        input! { c: u64 }
        *ci = c;
    }
    for (_, pi) in &mut cp {
        input! { p: u64 }
        *pi = p;
    }

    cp.sort_unstable();
    cp.dedup_by_key(|&mut (ci, _)| ci);
    if cp.len() < k {
        println!("-1");
        return;
    }
    cp.sort_unstable_by_key(|&(_, pi)| pi);
    let ans = cp.iter().take(k).map(|&(_, pi)| pi).sum::<u64>();
    println!("{ans}");
}
