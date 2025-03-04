use proconio::input;

fn main() {
    input! {
        n: u64,
    }
    let n = n + 1;
    let mut digits = vec![];
    {
        let mut n = n;
        while n > 0 {
            digits.push((n % 10) as u8);
            n /= 10;
        }
    }

    let m = digits.len();

    const REM_MAX: usize = 9*15;
    const MOD_MAX: usize = 9 * 15 + 1;
    // dp[i][j][k]: 桁和がiである数のうちjで割った余りがkとなるものの数
    let mut dp = [[[0; REM_MAX]; MOD_MAX]; MOD_MAX];
    let mut dp_nxt;
    for modulo in 1..MOD_MAX {
        dp[0][modulo][0] = 1;
    }

    let (digit_sums, ns) = {
        let mut v = vec![0; m];
        let mut w = vec![n; m];
        for i in (0..m - 1).rev() {
            v[i] = v[i + 1] + digits[i + 1];
        }
        let mut p10 = 1;
        for wi in &mut w {
            p10 *= 10;
            *wi = *wi / p10 * p10;
        }
        (v, w)
    };

    let mut ans = 0u64;
    let mut pow10 = 1;
    for power in 0..m {
        // 10^power未満 -> 10^(power + 1)未満への遷移
        dp_nxt = dp;
        for dig in 1..=9 {
            // dp_nxt: dig * 10^power未満における解
            if digits[power] == dig {
                let base_dig_sum = digit_sums[power];
                let base_n = ns[power];
                for dig_sum in 0..(power + 1) * 9 {
                    let modulo = base_dig_sum as usize + dig_sum;
                    if modulo == 0 {
                        continue;
                    }
                    ans += dp_nxt[dig_sum][modulo][(modulo - (base_n as usize) % modulo) % modulo];
                }
            }
            for dig_sum in 0..=power * 9 {
                for modulo in 1..MOD_MAX {
                    for rem in 0..modulo {
                        let dig = dig as usize;
                        dp_nxt[dig_sum + dig][modulo][(rem + dig * pow10) % modulo] += dp[dig_sum][modulo][rem];
                    }
                }
            }
        }
        pow10 *= 10;
        dp = dp_nxt;
    }
    println!("{ans}");
}
