use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [[u32]; n],
    }
    a.iter_mut().for_each(|nums| nums.sort_unstable());

    let mut ans = 0.0f64;
    for i in 0..n - 1 {
        let nums1 = &a[i];
        for j in i + 1..n {
            let nums2 = &a[j];
            let mut i = 0;
            let mut j = 0;
            let mut sum = 0;
            loop {
                if i >= nums1.len() || j >= nums2.len() {
                    break;
                }
                let ni = nums1[i];
                let nj = nums2[j];
                if ni > nj {
                    j += 1;
                    continue;
                }
                if ni < nj {
                    i += 1;
                    continue;
                }
                let x = ni;
                let mut c1 = 0u64;
                while i < nums1.len() && nums1[i] == x {
                    c1 += 1;
                    i += 1;
                }
                let mut c2 = 0;
                while j < nums2.len() && nums2[j] == x {
                    c2 += 1;
                    j += 1;
                }
                sum += c1 * c2;
            }
            // eprintln!("{sum}");
            let p = sum as f64 / (nums1.len() * nums2.len()) as f64;
            ans = ans.max(p);
        }
    }
    println!("{ans}");
}
