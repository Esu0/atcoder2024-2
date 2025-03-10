use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut b: [i64; n],
        mut w: [i64; m],
    }

    b.sort_unstable_by_key(|&x| -x);
    w.sort_unstable_by_key(|&x| -x);

    let mut sum_w = 0;
    let mut sum_b = 0;
    let mut max_w = 0;
    let mut ans = 0;
    for i in 0..n {
        // i + 1 個選ぶ
        if i < m {
            sum_w += w[i];
            max_w = max_w.max(sum_w);
        }
        sum_b += b[i];
        ans = ans.max(sum_b + max_w);
    }
    println!("{ans}");
}
