use proconio::input;

fn main() {
    input! { t: usize }
    'next: for _ in 0..t {
        input! {
            n: usize,
            mut a: [u32; n],
        }
        a.sort_unstable();
        let mut odd_cnt = 0;
        for &ai in &a {
            if odd_cnt < ai - 1 {
                println!("Yes");
                continue 'next;
            }
            if ai % 2 == 1 {
                odd_cnt += 1;
            }
        }
        println!("No");
    }
}
