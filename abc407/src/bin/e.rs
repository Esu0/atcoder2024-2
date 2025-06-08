use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    let mut queue = std::collections::BinaryHeap::new();
    for _ in 0..t {
        input! {
            n: usize,
            a: [u64; n * 2],
        }

        queue.clear();
        let mut ans = a[0];
        for i in 0..n - 1 {
            queue.push(a[2 * i + 1]);
            queue.push(a[2 * i + 2]);
            ans += queue.pop().unwrap();
        }
        println!("{ans}");
    }
}
