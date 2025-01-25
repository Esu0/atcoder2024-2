use proconio::input;
use util::upper_bound;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [u64; n],
    }

    let a_max = *a.last().unwrap();
    let mut cumsum = Vec::with_capacity(n + 1);
    cumsum.push(0);
    cumsum.extend(a.iter().copied());

    for i in 0..n {
        cumsum[i + 1] += cumsum[i];
    }

    // 偶数番目のみ足したときの累積和
    let mut cumsum_even = Vec::with_capacity(n + 1);
    cumsum_even.push(0);
    let mut s = 0;
    for (i, &ai) in a.iter().enumerate() {
        if i % 2 != 0 {
            s += ai;
        }
        cumsum_even.push(s);
    }

    for _ in 0..q {
        input! { x: u64 }
        let k = upper_bound(0..=a_max, |y| {
            let l = a.partition_point(|&ai| ai <= x.saturating_sub(y));
            let r = a.partition_point(|&ai| ai < x + y);
            // n - r >= r - l
            n - r + l >= r
        }) - 1;
        let lres = a.binary_search(&x.saturating_sub(k));
        let r = a.partition_point(|&ai| ai < x + k);
        let mut l = lres.unwrap_or_else(|e| e);
        if lres.is_ok() && n - r < r - l {
            l += 1;
        }
        let d = (n - r) - (r - l);
        assert!(d == 0 || d == 1);

        let mut ans = cumsum[n] - cumsum[r];
        if (l + d) % 2 == 0 {
            ans += cumsum_even[l];
        } else {
            ans += cumsum[l] - cumsum_even[l];
        }
        println!("{ans}");
    }
}
