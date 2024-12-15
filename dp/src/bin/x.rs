use proconio::input;

fn main() {
    input! {
        n: usize,
        mut wsv: [(usize, usize, u64); n],
    }
    wsv.sort_unstable_by_key(|&(w, s, _)| w + s);
    let max_s = wsv.iter().map(|&wsv| wsv.1).max().unwrap();
    let mut dpi = vec![0; max_s + 1];
    let mut dpi_next = Vec::with_capacity(max_s + 1);
    let mut ans = 0;
    for &(wi, si, vi) in &wsv {
        dpi_next.clone_from(&dpi);
        for j in 0..=max_s {
            if j + wi <= max_s && j <= si {
                dpi_next[j + wi] = dpi_next[j + wi].max(dpi[j] + vi);
            }
            if j <= si {
                ans = ans.max(dpi[j] + vi);
            }
        }
        dpi.clone_from(&dpi_next);
    }
    println!("{ans}");
}
