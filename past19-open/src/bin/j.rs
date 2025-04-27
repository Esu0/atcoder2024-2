use proconio::input;

fn dfs(groups: &mut Vec<(u8, u32)>, a: &[u32], i: usize, f: &mut impl FnMut(&[(u8, u32)])) {
    let n = a.len();
    if i == n {
        f(&groups[..]);
        return;
    }

    for j in 0..groups.len() {
        if groups[j].0 < 3 {
            groups[j].0 += 1;
            groups[j].1 += a[i];
            dfs(groups, a, i + 1, f);
            groups[j].0 -= 1;
            groups[j].1 -= a[i];
        }
    }

    if groups.len() < n / 3 {
        groups.push((1, a[i]));
        dfs(groups, a, i + 1, f);
        groups.pop();
    }
}

fn main() {
    input! {
        n: usize,
        a: [u32; n * 3],
    }

    let mut ans = u32::MAX;
    let mut groups = Vec::with_capacity(n);
    dfs(&mut groups, &a, 0, &mut |g| {
        let mx = g.iter().map(|&(_, x)| x).max().unwrap();
        let mn = g.iter().map(|&(_, x)| x).min().unwrap();
        ans = ans.min(mx - mn);
    });
    println!("{ans}");
}
