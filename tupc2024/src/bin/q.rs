use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            mut lr: [(u64, u64); n],
        }

        lr.sort_unstable();
        // lr.sort_unstable_by_key(|&(li, ri)| (li, ri));

        let mut mxr = 0;
        let mut all = 0;
        let mut sum = 0;
        for &(li, ri) in &lr {
            all += ri.saturating_sub(li.max(mxr));
            mxr = mxr.max(ri);
            sum += ri - li;
        }
        println!("{}", sum - all);
    }
}
