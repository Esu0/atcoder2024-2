use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }
    // let cum_sum = {
    //     let mut v = vec![0];
    //     v.extend_from_slice(&a);
    //     for i in 0..n {
    //         v[i + 1] += v[i];
    //     }
    //     v
    // };
    let sum = a.iter().copied().sum::<u64>();
    // let mut base = cum_sum.iter().copied().sum::<u64>();
    // let mut buf = a.iter().map(|&ai| (0, ai)).collect::<Vec<_>>();
    let mut map = HashMap::from([(0, (n, sum))]);
    let mut sums = [0; 26];
    for i in 0..25 {
        let prev_mask = (1u64 << i) - 1;
        let mask = (1u64 << (i + 1)) - 1;
        for &ai in &a {
            if ai & (1 << i) != 0 {
                let key = ai & prev_mask;
                let val = map.get_mut(&key).unwrap();
                val.1 -= ai;
                val.0 -= 1;
                if val.0 == 0 {
                    map.remove(&key);
                }
                map.entry(ai & mask).and_modify(|v| { v.0 += 1; v.1 += ai}).or_insert((1, ai));
            }
        }
        let mut s = 0;
        for &ai in &a {
            let key = mask & ai;
            if ai & prev_mask != 0 {
                let nk = (!key).wrapping_add(1) & mask;
                if let Some(&val) = map.get(&nk) {
                    s += val.0 as u64 * ai + val.1;
                }
            }
        }
        s /= 2;
        // eprintln!("{map:?}");
        // eprintln!("{s}");
        // eprintln!("{s}");
        if let Some(&s0) = map.get(&0) {
            s += s0.1 * (s0.0 - 1) as u64;
        }
        if let Some(&s0) = map.get(&(1 << i)) {
            s += s0.1 * (s0.0 - 1) as u64;
        }
        sums[i + 1] = s;
        // eprintln!("{s}");
    }
    let s0 = sum * (n - 1) as u64;
    sums[0] = s0;
    for i in 0..25 {
        sums[i] -= sums[i + 1];
    }
    // eprintln!("{sums:?}");
    let mut div = 1;
    let mut ans = 0;
    for &si in &sums {
        ans += si / div;
        div *= 2;
    }
    for &ai in &a {
        ans += ai >> ai.trailing_zeros();
    }
    println!("{ans}");
    // eprintln!("{sums:?}");
}
