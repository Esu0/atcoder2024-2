use proconio::input;

fn main() {
    input! {
        n: usize,
        mut s: u64,
        a: [u64; n],
    }

    let sum = a.iter().copied().sum::<u64>();
    s %= sum;
    if s == 0 {
        println!("Yes");
        return;
    }
    let a = a.iter().copied().chain(a.iter().copied()).collect::<Vec<_>>();
    let cum_sum = {
        let mut v = vec![0];
        v.extend_from_slice(&a);
        for i in 0..a.len() {
            v[i + 1] += v[i];
        }
        v
    };
    for i in 0..a.len() {
        if cum_sum[i + 1..].binary_search_by_key(&s, |&x| {
            x - cum_sum[i]
        }).is_ok() {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
