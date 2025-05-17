use proconio::input;
type MInt = util::ModInt<998244353>;

fn main() {
    input! {
        t: usize,
    }

    // dp_cnt[i][j]: [0, 2^i)のうち、popcountがjであるものの個数
    // dp_sum[i][j]: [0, 2^i)のうち、popcountがjであるものの総和
    let mut dp_cnt = [[0u64; 61]; 61];
    let mut dp_sum = [[MInt::new(0); 61]; 61];

    dp_cnt[0][0] = 1;
    for i in 1..=60 {
        dp_cnt[i][0] = dp_cnt[i - 1][0];
        // dp_sum[i][0] = dp_sum[i - 1][0];
        for j in 1..=i {
            dp_cnt[i][j] = dp_cnt[i - 1][j] + dp_cnt[i - 1][j - 1];
            dp_sum[i][j] = dp_sum[i - 1][j] + dp_sum[i - 1][j - 1] + MInt::new(1 << (i - 1)) * MInt::new(dp_cnt[i - 1][j - 1] as _);
        }
    }
    for _ in 0..t {
        input! { n: u64, mut k: usize }
        let n = n + 1;
        let mut ans = MInt::new(0);
        let mut m = 0u64;
        for i in (0..=60).rev() {
            if (n >> i) & 1 != 0 {
                ans += dp_sum[i][k] + MInt::new(m as _) * MInt::new(dp_cnt[i][k] as _);
                // eprintln!("{ans}");
                m += 1 << i;
                if k == 0 {
                    break;
                }
                k -= 1;
            }
        }
        println!("{ans}");
    }
}
