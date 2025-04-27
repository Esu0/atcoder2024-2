use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [u64; n],
    }

    if n % 2 == 0 {
        a.sort_unstable();
        let ans = a[n / 2..].iter().copied().sum::<u64>() - a[..n / 2].iter().copied().sum::<u64>();
        println!("{ans}");
    } else {
        let a = a;
        let mut s1 = BinaryHeap::new();
        let mut s2 = BinaryHeap::new();
        let mut sum1 = 0;
        let mut sum2 = 0;

        let mut i = n;
        let mut arr = vec![0; n];
        while i > 1 {
            i -= 2;
            if s1.peek().is_some_and(|&x| x < a[i]) {
                s2.push(Reverse(a[i]));
                sum2 += a[i];
            } else {
                s1.push(a[i]);
                sum1 += a[i];
            }
            if s1.peek().is_some_and(|&x| x < a[i + 1]) {
                s2.push(Reverse(a[i + 1]));
                sum2 += a[i + 1];
            } else {
                s1.push(a[i + 1]);
                sum1 += a[i + 1];
            }
            if s1.len() > s2.len() {
                let x = s1.pop().unwrap();
                sum1 -= x;
                sum2 += x;
                s2.push(Reverse(x));
            } else if s1.len() < s2.len() {
                let Reverse(x) = s2.pop().unwrap();
                sum1 += x;
                sum2 -= x;
                s1.push(x);
            }
            arr[i - 1] = sum2 - sum1;
        }
        // eprintln!("{arr:?}");
        s1.clear();
        s2.clear();
        sum1 = 0;
        sum2 = 0;
        let mut i = 0;
        let mut ans = arr[0];
        while i < n - 1 {
            if s1.peek().is_some_and(|&x| x < a[i]) {
                s2.push(Reverse(a[i]));
                sum2 += a[i];
            } else {
                s1.push(a[i]);
                sum1 += a[i];
            }
            if s1.peek().is_some_and(|&x| x < a[i + 1]) {
                s2.push(Reverse(a[i + 1]));
                sum2 += a[i + 1];
            } else {
                s1.push(a[i + 1]);
                sum1 += a[i + 1];
            }
            // eprintln!("{sum1} {sum2}");
            if s1.len() > s2.len() {
                let x = s1.pop().unwrap();
                sum1 -= x;
                sum2 += x;
                s2.push(Reverse(x));
            } else if s1.len() < s2.len() {
                let Reverse(x) = s2.pop().unwrap();
                sum1 += x;
                sum2 -= x;
                s1.push(x);
            }
            // eprintln!("{sum1} {sum2}");
            assert_eq!(s1.len(), s2.len());
            // eprintln!("{sum1} {sum2}");
            ans = ans.max(sum2 - sum1 + arr[i + 2]);
            i += 2;
        }

        println!("{}", ans);
    }
}
