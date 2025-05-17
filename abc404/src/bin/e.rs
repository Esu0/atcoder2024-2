use proconio::input;

fn main() {
    input! {
        n: usize,
        c: [usize; n - 1],
        a: [usize; n - 1],
    }

    let mut a = {
        let mut v = Vec::with_capacity(n);
        v.push(0);
        v.extend_from_slice(&a);
        v
    };
    let mut ans = 0;
    for i in (0..n).rev() {
        if a[i] == 0 {
            continue;
        }
        a[i] = 0;
        let mut cmin = i;
        let mut min = i;
        let mut j = i;
        let mut cnt = 0usize;
        while j != 0 && a[j] != 1 {
            if cmin > j - c[j - 1] {
                cmin = j - c[j - 1];
            }
            if j == min {
                min = cmin;
                cnt += 1;
            }
            j -= 1;
        }
        ans += cnt;
    }
    println!("{ans}");
}
