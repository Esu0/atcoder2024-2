use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            mut h: usize,
            mut w: usize,
            mut s: [proconio::marker::Bytes; h],
        }

        if w > h {
            let new_s = (0..w).map(|i| (0..h).map(|j| s[j][i]).collect::<Vec<_>>()).collect::<Vec<_>>();
            (h, w) = (w, h);
            s = new_s;
        }

        let cum_sum = s.iter().map(|si| {
            let mut v = std::iter::once(0u32).chain(si.iter().map(|&sij| if sij == b'#' { 1 } else { 0 })).collect::<Vec<_>>();
            for i in 0..w {
                v[i + 1] += v[i];
            }
            v
        }).collect::<Vec<_>>();

        let mut cnt = HashMap::new();
        let mut add = 0;
        let insert = |map: &mut HashMap<i32, u32>, val: i32| {
            *map.entry(val).or_insert(0) += 1;
        };
        let mut ans = 0u64;
        for l in 0..w {
            for r in l + 1..=w {
                cnt.clear();
                for row in &cum_sum {
                    let a = (r - l) as i32 - ((row[r] - row[l]) * 2) as i32;
                    insert(&mut cnt, -add);
                    add += a;
                    ans += cnt.get(&-add).copied().unwrap_or(0) as u64;
                }
            }
        }
        println!("{ans}");
    }
}
