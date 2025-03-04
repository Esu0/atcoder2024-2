use proconio::input;
use std::{cmp::Reverse, collections::BinaryHeap};

fn main() {
    input! {
        n: usize,
        w: [u64; n],
        lr: [(usize, usize); n],
        q: usize,
        st: [(usize, usize); q],
    }

    let mut ilr = lr.iter().enumerate().map(|(i, &(l, r))| (i, l - 1, r)).collect::<Vec<_>>();
    ilr.sort_unstable_by_key(|ilr| ilr.1);
    let mut upper = vec![usize::MAX; n];
    let mut ind = vec![usize::MAX; n];
    for (j, &(i, _, ri)) in ilr.iter().enumerate() {
        ind[i] = j;
        upper[j] = ilr.partition_point(|x| x.1 < ri);
    }

    let w = {
        let mut v = vec![u64::MAX; n];
        for (j, &(i, _, _)) in ilr.iter().enumerate() {
            v[j] = w[i];
        }
        v
    };
    // eprintln!("upper: {upper:?}");
    // eprintln!("w: {w:?}");
    let minw = {
        let mut v = vec![u64::MAX; n + 1];
        for j in (0..n).rev() {
            v[j] = v[j + 1].min(w[j]);
        }
        v
    };

    // eprintln!("minw: {minw:?}");
    let minx = {
        let mut v = vec![u64::MAX; n];
        let mut queue = BinaryHeap::new();
        let mut mn = u64::MAX;
        for j in 0..n {
            while queue.peek().is_some_and(|&(Reverse(x), _)| x <= j) {
                let Some((_, wj)) = queue.pop() else { unreachable!() };
                mn = mn.min(wj);
            }
            v[j] = mn;
            queue.push((Reverse(upper[j]), w[j]));
        }
        v
    };

    // eprintln!("minx: {minx:?}");
    for &(si, ti) in &st {
        let mut ind_s = ind[si - 1];
        let mut ind_t = ind[ti - 1];
        if ind_s > ind_t {
            (ind_s, ind_t) = (ind_t, ind_s);
        }
        // eprintln!("s: {ind_s}, t: {ind_t}");
        if upper[ind_s] <= ind_t {
            println!("{}", w[ind_s] + w[ind_t]);
            continue;
        }
        let mut ans = u64::MAX;
        ans = ans.min(minx[ind_s].saturating_add(w[ind_s] + w[ind_t]));
        let umax = upper[ind_s].max(upper[ind_t]);
        if umax < n {
            ans = ans.min(minw[umax].saturating_add(w[ind_s] + w[ind_t]));
        }
        if upper[ind_s] < n {
            ans = ans.min(minx[ind_t].saturating_add(minw[upper[ind_s]]).saturating_add(w[ind_s] + w[ind_t]));
        }
        println!("{}", ans as isize);
    }
}
