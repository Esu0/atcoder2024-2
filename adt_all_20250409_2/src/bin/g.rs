use std::collections::HashSet;

use proconio::input;

fn solve(groups: &mut Vec<u64>, a: &[u64], i: usize, ans: &mut HashSet<u64>) {
    let n = a.len();
    if n == i {
        ans.insert(groups.iter().copied().fold(0, |acc, x| acc ^ x));
        return;
    }

    for j in 0..groups.len() {
        groups[j] += a[i];
        solve(groups, a, i + 1, ans);
        groups[j] -= a[i];
    }
    groups.push(a[i]);
    solve(groups, a, i + 1, ans);
    groups.pop();
}

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }

    let mut ans = HashSet::new();
    let mut groups = vec![];
    solve(&mut groups, &a, 0, &mut ans);
    println!("{}", ans.len());
}
