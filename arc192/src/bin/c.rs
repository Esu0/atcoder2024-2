use std::{cmp::Reverse, sync::atomic::{AtomicUsize, Ordering}};


static REM: AtomicUsize = AtomicUsize::new(0);
fn query(s: usize, t: usize) -> u64 {
    let s = s + 1;
    let t = t + 1;
    if REM.load(Ordering::Relaxed) == 0 {
        panic!("QLE");
    }
    REM.fetch_add(1, Ordering::Relaxed);
    assert_ne!(s, t);
    println!("? {s} {t}");
    proconio::input_interactive!{ x: i64 }
    if x == -1 {
        panic!();
    }
    x as _
}

fn main() {
    proconio::input_interactive! {
        n: usize,
    }
    REM.store(2 * n, Ordering::Relaxed);

    let base = query(0, 1);
    // let mut results = vec![(u64::MAX, u64::MAX); n];
    let mut left = vec![];
    let mut right = vec![];
    let mut mid = vec![];
    for i in 2..n {
        let q0 = query(0, i);
        let q1 = query(1, i);
        if q0 < base && q1 < base {
            mid.push((q0, q1, i));
        } else if q0 > q1 {
            right.push((q0, q1, i));
        } else {
            assert_ne!(q0, q1);
            left.push((q0, q1, i));
        }
    }

    left.sort_unstable_by_key(|x| Reverse(x.0));
    right.sort_unstable_by_key(|x| x.1);
    mid.sort_unstable_by_key(|x| x.0);

    let mut a = vec![u64::MAX; n];
    if !left.is_empty() {
        let ll = left.last().unwrap();
        a[left.len()] = ll.0 + base - ll.1;
        if mid.is_empty() {
            a[left.len() + 1] = ll.1 - ll.0;
        }
        for i in 0..left.len() - 1 {
            a[i] = left[i].0 - left[i + 1].0;
        }
        a[left.len() - 1] = left.last().unwrap().1 - base;
    }
    let offset = left.len() + 1;
    for (i, &x) in mid.iter().enumerate() {
        a[offset + i] = x.0 + x.1 - base;
    }

    {// right
        let offset = left.len() + mid.len() + 2;
        let mut tmp = std::iter::once(base).chain(right.iter().map(|x| x.0)).collect::<Vec<_>>();
        for i in (0..tmp.len() - 1).rev() {
            tmp[i + 1] -= tmp[i];
        }

        for i in 0..right.len() {
            a[i + offset] = tmp[i + 1];
        }
    }

    if !right.is_empty() {
        let rr = right.first().unwrap();
        // a[left.len() + mid.len() + 1]
    }
    let mut p = vec![usize::MAX; n];
    let mut i = 0;
    for &x in &left {
        p[x.2] = i;
        i += 1;
    }
    p[0] = i;
    i += 1;
    for &x in &mid {
        p[x.2] = i;
        i += 1;
    }
    p[1] = i;
    i += 1;
    for &x in &mid {
        p[x.2] = i;
        i += 1;
    }

}
