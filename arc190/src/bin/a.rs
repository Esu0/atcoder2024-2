use std::cmp::Reverse;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        lr: [(usize, usize); m],
    }

    if let Some(i) = lr.iter().position(|lr| *lr == (1, n)) {
        // K = 1
        println!("1");
        for _ in 0..i {
            print!("0 ");
        }
        print!("1 ");
        for _ in i + 1..m {
            print!("0 ");
        }
        println!();
        return;
    }

    let (mnr, mnri) = lr
        .iter()
        .enumerate()
        .map(|(i, &(_, ri))| (ri, i))
        .min()
        .unwrap();
    let (mxl, mxli) = lr
        .iter()
        .enumerate()
        .map(|(i, &(li, _))| (li, i))
        .max()
        .unwrap();
    if mnri != mxli && mnr < mxl {
        println!("2");
        for i in 0..m {
            if i == mnri || i == mxli {
                print!("2 ");
            } else {
                print!("0 ");
            }
        }
        println!();
        return;
    }

    if let Some((mxr, mxri)) = lr
        .iter()
        .enumerate()
        .filter(|&(_, &(li, _))| li == 1)
        .map(|(i, &(_, ri))| (ri, i))
        .max()
    {
        if let Some((mnl, mnli)) = lr
            .iter()
            .enumerate()
            .filter(|&(_, &(_, ri))| ri == n)
            .map(|(i, &(li, _))| (li, i))
            .min()
        {
            if mnl <= mxr {
                println!("2");
                for i in 0..m {
                    if i == mxri || i == mnli {
                        print!("1 ");
                    } else {
                        print!("0 ");
                    }
                }
                println!();
                return;
            }
        }
    }
    let mut lri = lr
        .iter()
        .enumerate()
        .map(|(i, &(li, ri))| (li, ri, i))
        .collect::<Vec<_>>();
    lri.sort_unstable_by_key(|x| (x.0, Reverse(x.1)));

    let (mut prev_r, mut prev_ri) = (lri[0].1, lri[0].2);
    for &(_, ri, i) in &lri[1..] {
        if prev_r >= ri {
            println!("2");
            for j in 0..m {
                if j == i {
                    print!("2 ");
                } else if j == prev_ri {
                    print!("1 ");
                } else {
                    print!("0 ");
                }
            }
            println!();
            return;
        }
        prev_r = ri;
        prev_ri = i;
    }

    if m <= 2 {
        println!("-1");
        return;
    }

    let mut lri = [
        (lr[0].0, lr[0].1, 0usize),
        (lr[1].0, lr[1].1, 1),
        (lr[2].0, lr[2].1, 2),
    ];
    lri.sort_unstable_by_key(|x| x.0);
    assert!(lri[0].0 < lri[1].0 && lri[1].0 < lri[2].0);
    assert!(lri[0].1 < lri[1].1 && lri[1].1 < lri[2].1);
    println!("3");
    for i in 0..m {
        if i == lri[0].2 || i == lri[2].2 {
            print!("2 ");
        } else if i == lri[1].2 {
            print!("1 ");
        } else {
            print!("0 ");
        }
    }
}
