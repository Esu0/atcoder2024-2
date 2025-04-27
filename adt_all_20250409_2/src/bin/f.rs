use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,

        s: [proconio::marker::Bytes; h],
        t: [proconio::marker::Bytes; h],
    }

    let mut sc = vec![vec![0; h]; w];
    let mut tc = vec![vec![0; h]; w];
    for (i, row) in s.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            sc[j][i] = cell;
            tc[j][i] = t[i][j];
        }
    }
    sc.sort_unstable();
    tc.sort_unstable();
    if sc == tc {
        println!("Yes");
    } else {
        println!("No");
    }
}
