use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [u8; n],
        b: [u8; n],
        p: [usize; n],
        q: [usize; n],
    }
    let x = x - 1;
    // fg1[i]: (頂点iから出ていく辺, 頂点iに入る辺)
    let mut fg1 = vec![(usize::MAX, usize::MAX); n];
    let mut fg2 = vec![(usize::MAX, usize::MAX); n];
    for (i, &pi) in p.iter().enumerate() {
        let pi = pi - 1;
        fg1[i].0 = pi;
        fg1[pi].1 = i;
    }
    for (i, &qi) in q.iter().enumerate() {
        let qi = qi - 1;
        fg2[i].0 = qi;
        fg2[qi].1 = i;
    }
    let a_sum = a.iter().map(|&ai| ai as usize).sum::<usize>();
    let b_sum = b.iter().map(|&bi| bi as usize).sum::<usize>();
    {
        let mut cur = x;
        let mut sum = a[cur] as usize;
        while fg1[cur].0 != x {
            cur = fg1[cur].0;
            sum += a[cur] as usize;
        }
        if sum != a_sum {
            println!("-1");
            return;
        }
    }
    {
        let mut cur = x;
        let mut sum = b[cur] as usize;
        while fg2[cur].0 != x {
            cur = fg2[cur].0;
            sum += b[cur] as usize;
        }
        if sum != b_sum {
            println!("-1");
            return;
        }
    }
    let mut v1 = vec![];
    let mut v2 = vec![];
    {
        let mut cur = x;
        while fg1[cur].0 != x {
            cur = fg1[cur].0;
            if a[cur] == 1 || !v1.is_empty() {
                v1.push(cur);
            }
        }
    }
    {
        let mut cur = x;
        while fg2[cur].0 != x {
            cur = fg2[cur].0;
            if b[cur] == 1 || !v2.is_empty() {
                v2.push(cur);
            }
        }
    }
    // eprintln!("{v1:?}");
    // eprintln!("{v2:?}");

    let mut map = vec![0; n];
    for (i, &x) in v2.iter().enumerate() {
        map[x] = i + 1;
    }
    let mut arr = vec![0];
    arr.extend(v1.iter().map(|&x| map[x]));

    let mut dp = Vec::<usize>::new();
    for &vi in arr.iter() {
        let j = dp.partition_point(|&x| x < vi);
        if j == dp.len() {
            dp.push(vi);
        } else {
            dp[j] = vi;
        }
    }
    // eprintln!("{dp:?}");
    let ans = v1.len() + v2.len() - (dp.len() - 1);
    println!("{ans}");
}
