use proconio::input;

fn solve(dep: usize, c: &[u64], buf: &mut [usize], fee: u64, f: &mut impl FnMut(&[usize], u64)) {
    let n = c.len();
    if dep == n {
        f(&*buf, fee);
        return;
    }
    for cnt in 0..3 {
        buf[dep] = cnt;
        solve(dep + 1, c, buf, fee + c[dep] * cnt as u64, f);
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        c: [u64; n],
        a: [[usize]; m],
    }

    let mut animal = vec![vec![]; n];
    for (i, ai) in a.iter().enumerate() {
        for &aij in ai {
            animal[aij - 1].push(i);
        }
    }
    let mut buf = vec![0; n];
    let mut ans = u64::MAX;
    let mut set = vec![0; m];
    // eprintln!("{animal:?}");
    solve(0, &c, &mut buf, 0, &mut |b, f| {
        set.fill(0);
        for (i, &bi) in b.iter().enumerate() {
            if bi == 0 {
                continue;
            }
            for &ai in &animal[i] {
                set[ai] += bi;
            }
        }
        // eprintln!("{b:?}");
        // eprintln!("{set:?}");
        if set.iter().all(|&si| si >= 2) {
            ans = ans.min(f);
        }
    });
    assert_ne!(ans, u64::MAX);
    println!("{ans}");
}
