use proconio::input;

fn main() {
    input! {
        n: usize,
        w: usize,
        xy: [(usize, usize); n],
        q: usize,
    }

    let mut columns = vec![vec![(usize::MAX, usize::MAX)]; w];

    for (i, &(xi, yi)) in xy.iter().enumerate() {
        columns[xi - 1].push((yi, i));
    }

    columns.iter_mut().for_each(|col| col.sort_unstable_by_key(|x| x.0));
    let mut del_t = vec![usize::MAX; n];
    let mut i = 0;
    loop {
        let mut max = 0;
        for j in 0..w {
            max = max.max(columns[j][i].0);
        }
        if max == usize::MAX {
            break;
        }
        for j in 0..w {
            del_t[columns[j][i].1] = max;
        }
        i += 1;
    }
    // eprintln!("{del_t:?}");
    for _ in 0..q {
        input! { t: usize, a: usize }
        if t >= del_t[a - 1] {
            println!("No");
        } else {
            println!("Yes");
        }
    }
}
