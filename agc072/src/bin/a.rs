use proconio::input;

fn main() {
    input! { t: usize }

    for _ in 0..t {
        input! {
            n: usize,
            d: u64,
            tx: [(u64, u64); n],
        }
    }
}
