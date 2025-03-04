use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut ans = 0usize;
    for i in 0..n - 4 {
        let s = a[i];
        for j in i + 1..n - 3 {
            let s = s + a[j];
            for k in j + 1..n - 2 {
                let s = s + a[k];
                for l in k + 1..n - 1 {
                    let s = s + a[l];
                    for m in l + 1..n {
                        if s + a[m] == 1000 {
                            ans += 1;
                        }
                    }
                }
            }
        }
    }
    println!("{ans}");
}
