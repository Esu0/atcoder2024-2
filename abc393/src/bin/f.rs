use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [u32; n],
        rx: [(usize, u32); q],
    }

    let mut irx = rx.iter().copied().enumerate().collect::<Vec<_>>();

    irx.sort_unstable_by_key(|x| x.1.0);

    let mut ans = vec![usize::MAX; q];
    let mut dp = Vec::<u32>::new();
    let mut j = 0;
    for (i, &ai) in a.iter().enumerate() {
        let x = dp.partition_point(|&x| x < ai);
        if x == dp.len() {
            dp.push(ai);
        } else {
            dp[x] = ai;
        }
        while j < q && irx[j].1.0 == i + 1 {
            let t = dp.partition_point(|&x| x <= irx[j].1.1);
            ans[irx[j].0] = t;
            j += 1;
        }
    }
    ans.iter().for_each(|&x| println!("{x}"));
}
