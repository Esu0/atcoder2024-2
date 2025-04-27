use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [u32; n],
    }

    let mut ans = 1;
    for i in 0..n {
        for j in 1..n - i {
            let mut cnt = 1;
            let mut k = i + j;
            let b = h[i];
            while k < n && h[k] == b {
                cnt += 1;
                k += j;
            }
            ans = ans.max(cnt);
        }
    }
    println!("{ans}");
}
