use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        xy: [(usize, usize); q],
    }

    let mut dp = vec![Option::<Vec<bool>>::None; 8889];
    dp[0] = Some((0..n).map(|_| false).collect());

    let mut ng = vec![vec![]; n];
    for &(xi, yi) in &xy {
        ng[yi - 1].push(xi - 1);
    }

    let (mut b, mut c) = (vec![], vec![]);
    'end: for (i, &ai) in a.iter().enumerate() {
        for j in (0..=8888 - ai).rev() {
            if let Some(dpj) = &dp[j] {
                if ng[i].iter().any(|&x| dpj[x]) {
                    continue;
                }
                if let Some(dpk) = &dp[j + ai] {
                    for l in 0..=i {
                        if dpj[l] {
                            b.push(l + 1);
                        }
                        if dpk[l] {
                            c.push(l + 1);
                        }
                    }
                    b.push(i + 1);
                    break 'end;
                } else {
                    let mut tmp = dpj.clone();
                    tmp[i] = true;
                    dp[j + ai] = Some(tmp);
                }
            }
        }
    }
    assert!(!b.is_empty());

    assert_ne!(b, c);
    let s1 = b.iter().map(|&bi| a[bi - 1]).sum::<usize>();
    let s2 = c.iter().map(|&ci| a[ci - 1]).sum::<usize>();
    assert_eq!(s1, s2);

    for &(xi, yi) in &xy {
        assert!(!(b.binary_search(&xi).is_ok() && b.binary_search(&yi).is_ok()));
        assert!(!(c.binary_search(&xi).is_ok() && c.binary_search(&yi).is_ok()));
    }

    println!("{}", b.len());
    print!("{}", b[0]);
    for &bi in &b[1..] {
        print!(" {}", bi);
    }
    println!();
    println!("{}", c.len());
    print!("{}", c[0]);
    for &ci in &c[1..] {
        print!(" {}", ci);
    }
    println!();
}
