use proconio::input;

fn main() {
    input! {
        t: usize,
        nk: [(u64, u64); t],
    }


    for &(n, k) in &nk {
        let mut zeros = 0;
        {
            let mut n = n;
            while n != 1 {
                if n & 1 == 0 {
                    zeros += 1;
                }
                n >>= 1;
            }
        }
        if k > (1 << zeros) {
            println!("-1");
            continue;
        }
        let k = k - 1;
        let mut ans = n;
        let mut j = 0;
        for i in 0..zeros {
            while (n & (1 << j)) != 0 {
                j += 1;
            }
            ans |= ((k >> i) & 1) << j;
            j += 1;
        }
        println!("{ans}");
    }
}
