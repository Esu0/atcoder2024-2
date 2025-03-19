use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        p: [[usize; w]; h],
    }

    let mut set = HashMap::<usize, usize>::new();
    let mut ans = 0;
    for s in 1u32..(1 << h) {
        let pop = s.count_ones() as usize;
        set.clear();
        'nxt: for i in 0..w {
            let mut val = Option::<usize>::None;
            for j in 0..h {
                if s & (1 << j) != 0 {
                    if let Some(prev) = val {
                        if prev != p[j][i] {
                            continue 'nxt;
                        }
                    } else {
                        val = Some(p[j][i]);
                    }
                }
            }
            set.entry(val.unwrap()).and_modify(|c| *c += 1).or_insert(1);
        }
        let mut mx = 0;
        for &v in set.values() {
            mx = mx.max(v);
        }
        // if mx * pop == 9 {
        //     eprintln!("{set:?}");
        // }
        ans = ans.max(mx * pop);
    }
    println!("{ans}");
}
