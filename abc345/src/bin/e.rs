use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        cv: [(usize, i64); n],
    }

    let mut col_to_ind = vec![(usize::MAX, 0usize); n + 1];
    // let mut ind_to_col = vec![0; k + 1];
    // dp[i][j]: 末尾が色jとなるようにi個のボールを取り除いたときの残ったボールの価値の総和の最大値
    let mut dp = VecDeque::new();

    let chmax = |arr: &mut (Vec<i64>, usize, usize), i: usize, v: i64| {
        let (arr, o1, o2) = arr;
        if arr[i] >= v {
            return;
        }
        arr[i] = v;
        if *o1 == i {
            return;
        }
        if *o2 == i && arr[*o1] < v {
            *o2 = *o1;
            *o1 = i;
            return;
        }
        if arr[*o2] < v {
            *o2 = i;
            if arr[*o1] < v {
                *o2 = *o1;
                *o1 = i;
            }
        }
    };

    let get_max_except = |arr: &(Vec<i64>, usize, usize), i: usize| {
        if arr.1 == i {
            arr.0[arr.2]
        } else {
            arr.0[arr.1]
        }
    };

    let get_max = |arr: &(Vec<i64>, usize, usize)| {
        arr.0[arr.1]
    };

    let mut new_ind = 0;
    for (i, &(ci, vi)) in cv[..=k].iter().enumerate() {
        if col_to_ind[ci].1 == 0 {
            col_to_ind[ci] = (new_ind, 1);
            new_ind += 1;
        } else {
            col_to_ind[ci].1 += 1;
        }
        let col_ind = col_to_ind[ci].0;
        // ind_to_col[i] = ci;
        dp.push_front((vec![i64::MIN; k + 1], 0, 1));
        chmax(&mut dp[i], col_ind, vi);
        for j in 0..i {
            let mx = get_max_except(&dp[j + 1], col_ind);
            chmax(&mut dp[j], col_ind, mx + vi);
        }
    }

    let mut ind_list = (new_ind..=k).collect::<Vec<_>>();
    let mut buf = vec![i64::MIN; k + 1];
    for (&(c_prev, _), &(ci, vi)) in cv.iter().zip(&cv[k + 1..]) {
        col_to_ind[c_prev].1 -= 1;
        if col_to_ind[c_prev].1 == 0 {
            ind_list.push(col_to_ind[c_prev].0);
        }
        if col_to_ind[ci].1 == 0 {
            col_to_ind[ci].0 = ind_list.pop().unwrap();
        }
        col_to_ind[ci].1 += 1;
        let col_ind = col_to_ind[ci].0;
        // ind_to_col[del_ind] = ci;
        buf.fill(i64::MIN);
        dp.push_front((buf, 0, 1));
        for j in 0..=k {
            let mx = get_max_except(&dp[j + 1], col_ind);
            chmax(&mut dp[j], col_ind, mx + vi);
        }
        buf = dp.pop_back().unwrap().0;
    }

    let ans = get_max(&dp[k]);
    if ans < 0 {
        println!("-1");
    } else {
        println!("{ans}");
    }
}
