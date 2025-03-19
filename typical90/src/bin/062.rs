use proconio::input;

// fn permutation(buf: &mut [usize], i: usize, f: &mut impl FnMut(&[usize])) {
//     let n = buf.len();

//     if n == i {
//         f(&*buf);
//     }

//     for j in i..n {
//         buf.swap(i, j);
//         permutation(buf, i + 1, f);
//         buf.swap(i, j);
//     }
// }

// ab: 1-indexed
// ans: 0-indexed
fn check(ab: &[(usize, usize)], ans: &[usize]) -> bool {
    let n = ab.len();
    assert_eq!(n, ans.len());
    // false: White
    let mut col = vec![false; n];

    for &i in ans {
        let (ai, bi) = ab[i];
        if col[ai - 1] && col[bi - 1] {
            return false;
        }
        col[i] = true;
    }
    true
}

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    }

    let mut lst = vec![vec![]; n];

    let mut stack = Vec::with_capacity(n);
    let mut vis = vec![false; n];
    for (i, &(ai, bi)) in ab.iter().enumerate() {
        lst[ai - 1].push(i);
        lst[bi - 1].push(i);
        if i == ai - 1 || i == bi - 1 {
            stack.push(i);
            vis[i] = true;
        }
    }

    let mut ans = Vec::with_capacity(n);
    while let Some(u) = stack.pop() {
        ans.push(u);
        for &v in &lst[u] {
            if !vis[v] {
                vis[v] = true;
                stack.push(v);
            }
        }
    }

    ans.reverse();
    if vis.iter().copied().all(|c| c) {
        assert!(check(&ab, &ans));
        ans.iter().for_each(|x| println!("{}", *x + 1));
    } else {
        // let mut buf = (0..n).collect::<Vec<_>>();
        // permutation(&mut buf, 0, &mut |arr| {
        //     assert!(!check(&ab, arr));
        // });
        println!("-1");
    }
}
