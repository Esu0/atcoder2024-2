use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut c = vec![usize::MAX; 200001];
    for i in 1..=n {
        input! { mut ai: usize }
        while ai <= 200000 && c[ai] == usize::MAX {
            c[ai] = i;
            ai += 1;
        }
    }

    for _ in 0..m {
        input! { bi: usize }
        println!("{}", c[bi] as isize);
    }
}
