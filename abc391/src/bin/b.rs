use proconio::{input, marker};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [marker::Bytes; n],
        t: [marker::Bytes; m],
    }

    'outer: for a in 0..=n - m  {
        'nxt: for b in 0..=n - m {
            for i in 0..m {
                for j in 0..m {
                    if s[a + i][b + j] != t[i][j] {
                        continue 'nxt;
                    }
                }
            }
            println!("{} {}", a + 1, b + 1);
            break 'outer;
        }
    }
}
