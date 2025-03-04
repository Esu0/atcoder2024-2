use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut left = vec![usize::MAX; 1_000_001];

    let mut ans = usize::MAX;
    for (i, &ai) in a.iter().enumerate() {
        if left[ai] != usize::MAX {
            ans = ans.min(i - left[ai] + 1);
        }
        left[ai] = i;
    }

    println!("{}", ans as isize);
}
