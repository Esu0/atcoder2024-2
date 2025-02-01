use proconio::{input, marker};

fn main() {
    input! {
        n: usize,
        s: marker::Bytes,
    }
    
    let mut dpi = s.iter().map(|&c| (c - b'0', 1u64)).collect::<Vec<_>>();
    let mut dpi_nxt = Vec::new();
    let mut m = s.len();

    while m > 1 {
        let m_nxt = m / 3;
        dpi_nxt.clear();
        for i in 0..m_nxt {
            let mut arr = [dpi[i * 3], dpi[i * 3 + 1], dpi[i * 3 + 2]];
            arr.sort_unstable();
            let mut cnt0 = 0;
            for j in 0..3 {
                if arr[j].0 == 0 {
                    cnt0 += 1;
                }
            }

            let (b, c) = if cnt0 >= 2 {
                (0u8, arr[..cnt0 - 1].iter().map(|&(_, x)| x).sum::<u64>())
            } else {
                (1, arr[cnt0..2].iter().map(|&(_, x)| x).sum::<u64>())
            };
            dpi_nxt.push((b, c));
        }
        // eprintln!("{dpi_nxt:?}");
        dpi.clone_from(&dpi_nxt);
        m = m_nxt;
    }

    println!("{}", dpi[0].1);
}
