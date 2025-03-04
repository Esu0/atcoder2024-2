use proconio::input;

fn main() {
    input! {
        n: usize,
        p: u64,
        q: u64,
        a: [u64; n],
    }
    let mut ans = 0;
    for i in 0..n - 4 {
        let t = a[i];
        for j in i + 1..n - 3 {
            let t = t * a[j] % p;
            for k in j + 1..n - 2 {
                let t = t * a[k] % p;
                for l in k + 1..n - 1 {
                    let t = t * a[l] % p;
                    for m in l + 1..n {
                        let t = t * a[m] % p;
                        if t == q {
                            ans += 1;
                        }
                    }
                }
            }
        }
    }
    println!("{ans}");
}
