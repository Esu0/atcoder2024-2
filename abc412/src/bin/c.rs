use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    'next: for _ in 0..t {
        input! {
            n: usize,
            mut s: [u64; n],
        }

        let mut ans = 2;
        let last = s.pop().unwrap();
        let mut j = 0;
        let mut cur = s.remove(0);
        s.sort_unstable();
        while cur * 2 < last {
            let mut ok = false;
            while j < s.len() && cur * 2 >= s[j] {
                j += 1;
                ok = true;
            }
            if !ok {
                println!("-1");
                continue 'next;
            }
            cur = s[j - 1];
            ans += 1;
        }
        println!("{ans}");
    }
}
