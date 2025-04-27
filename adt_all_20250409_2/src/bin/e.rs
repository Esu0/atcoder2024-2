use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut prev = vec![usize::MAX; 1_000_001];
    let mut ans = usize::MAX;
    for (i, &ai) in a.iter().enumerate() {
        if prev[ai] == usize::MAX {
            prev[ai] = i;
        } else {
            ans = ans.min(i - prev[ai] + 1);
        }
    }
    println!("{}", ans as isize);
}
