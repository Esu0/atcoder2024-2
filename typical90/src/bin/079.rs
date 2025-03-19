use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        mut a: [[i64; w]; h],
        b: [[i64; w]; h],
    }

    let mut ans = 0;
    for i in 0..h - 1 {
        for j in 0..w - 1 {
            let diff = b[i][j] - a[i][j];
            a[i][j] += diff;
            a[i + 1][j] += diff;
            a[i][j + 1] += diff;
            a[i + 1][j + 1] += diff;
            ans += diff.unsigned_abs();
        }
        if a[i][w - 1] != b[i][w - 1] {
            println!("No");
            return;
        }
    }

    if a[h - 1] != b[h - 1] {
        println!("No");
    } else {
        println!("Yes");
        println!("{ans}");
    }

}
