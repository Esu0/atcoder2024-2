use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            k: usize,
            mut s: proconio::marker::Bytes,
        }

        let mut k = k % (4 * n);
        if k >= 2 * n {
            s.reverse();
            k -= 2 * n;
        }
        if k >= n {
            
        } else {

        }
    }
}
