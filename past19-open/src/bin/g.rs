use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [[usize; n]; n],
    }

    let (mut i0, mut j0) = (usize::MAX, usize::MAX);
    'outer: for i in 0..n {
        for j in 0..n {
            if a[i][j] == 0 {
                i0 = i;
                j0 = j;
                break 'outer;
            }
        }
    }

    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                if a[a[i][j]][k] != a[i][a[j][k]] {
                    
                }
            }
        }
    }
}
