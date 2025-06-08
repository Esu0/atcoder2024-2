use proconio::input;

fn judge(ab: &[(u64, u64)], ans: &[(u64, u64)]) {
    let mut a = ab.iter().map(|&(ai, _)| ai).collect::<Vec<_>>();
    for &(xi, yi) in ans {
        for ai in &mut a {
            *ai = (*ai + xi) % yi;
        }
    }
    for (&ai, &(_, bi)) in a.iter().zip(ab) {
        assert_eq!(ai, bi);
    }
}

fn main() {
    input! {
        n: usize,
    }

    let mut ab_orig = vec![(0, 0); n];
    for (dst, _) in ab_orig.iter_mut() {
        input! { ai: u64 }
        *dst = ai;
    }
    for (_, dst) in ab_orig.iter_mut() {
        input! { bi: u64 }
        *dst = bi;
    }
    ab_orig.sort_unstable_by_key(|&(ai, _)| ai);
    let mut v = Vec::with_capacity(n);
    let mut prev = ab_orig[0];
    v.push(prev);
    for &(ai, bi) in &ab_orig[1..] {
        if prev.0 == ai {
            if prev.1 != bi {
                println!("No");
                return;
            }
            continue;
        }
        prev = (ai, bi);
        v.push(prev);
    }

    println!("Yes");
    let mut ab = v;
    let n = ab.len();
    let m = 1_000_000_001;
    if n == 1 {
        if ab[0].0 == ab[0].1 {
            println!("0");
        } else {
            println!("1");
            println!("{} {}", (ab[0].1 + m - ab[0].0) % m, m);
        }
        return;
    }
    // eprintln!("{ab:?}");

    ab[0].1 += 2 * m * n as u64;
    for i in 1..n {
        ab[i].1 += 2 * m * i as u64;
    }

    let mut a = ab.iter().map(|&(ai, _)| ai).collect::<std::collections::VecDeque<_>>();

    let mut ans = Vec::with_capacity(n);
    for i in (1..n).rev() {
        let d = ab[(i + 1) % n].1 - ab[i].1;
        let min = a[0];
        let max = *a.back().unwrap();
        assert!(d > min);
        let x = d - min;
        let y = x + max;
        for ai in &mut a {
            *ai = (*ai + x) % y;
        }
        ans.push((x, y));
        let last = a.pop_back().unwrap();
        assert_eq!(last, 0);
        a.push_front(last);
    }
    let x = ab[1].1 % m;
    let y = m;
    for ai in &mut a {
        *ai = (*ai + x) % y;
    }
    ans.push((ab[1].1 % m, m));

    // eprintln!("{a:?}");
    println!("{}", ans.len());
    judge(&ab_orig, &ans);
    for &(xi, yi) in &ans {
        println!("{xi} {yi}");
    }
}
