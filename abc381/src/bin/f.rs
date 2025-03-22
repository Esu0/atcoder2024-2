use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u8; n],
    }

    let mut dp = vec![usize::MAX; 1 << 20];
    let mut queue = BinaryHeap::from([(Reverse(0), 0)]);
    let mut poss = vec![vec![]; 20];
    for (i, &ai) in a.iter().enumerate().rev() {
        poss[(ai - 1) as usize].push(i);
    }

    let mut c_mx = 0;
    while let Some((Reverse(c), u)) = queue.pop() {
        if dp[u] != usize::MAX {
            continue;
        }
        dp[u] = c;
        while c_mx < c {
            assert_eq!(poss[(a[c_mx] - 1) as usize].pop(), Some(c_mx));
            c_mx += 1;
        }

        for i in 0..20 {
            if u & (1 << i) == 0 {
                let v = u | (1 << i);
                let length = poss[i].len();
                if length >= 2 && dp[v] == usize::MAX {
                    // assert!(poss[i][length - 2] > c);
                    queue.push((Reverse(poss[i][length - 2] + 1), v));
                }
            }
        }
    }

    let mut ans = 0;
    for s in 0..1 << 20 {
        if dp[s] != usize::MAX {
            ans = ans.max(s.count_ones());
        }
    }
    println!("{}", ans * 2);
}
