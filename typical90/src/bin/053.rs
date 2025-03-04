use proconio::input_interactive;
use std::sync::atomic::{AtomicU8, Ordering::*};
use std::io::Write;

static Q: AtomicU8 = AtomicU8::new(0);

fn ask(i: usize) -> u32 {
    let q = Q.load(Relaxed);
    if q >= 15 {
        panic!();
    }
    Q.store(q + 1, Relaxed);
    println!("? {}", i + 1);
    std::io::stdout().flush().unwrap();
    input_interactive! { a: u32 }
    a
}

fn answer(a: u32) {
    println!("! {a}");
    std::io::stdout().flush().unwrap();
}
fn main() {
    input_interactive! {
        t: usize,
    }

    let magic = [1usize, 2, 4, 7, 12, 20, 33, 54, 88, 143, 232, 376, 609, 986, 1596];
    for _ in 0..t {
        Q.store(0, Relaxed);
        input_interactive! { n: usize }
        if n <= 3 {
            let mut mx = 0;
            for i in 0..n {
                mx = mx.max(ask(i));
            }
            answer(mx);
            continue;
        }

        let mut l = 0;
        // let mut r = n;
        let mut i = magic.partition_point(|&x| x < n);
        let mut al = ask(magic[i] - magic[i - 1] - 1);
        let mut ar = ask(magic[i - 1]);
        while i >= 2 {
            // assert_ne!(al, ar);
            if ar != u32::MAX && al < ar {
                l += magic[i] - magic[i - 1];
                al = ar;
                i -= 1;
                ar = if l + magic[i - 1] < n {
                    ask(l + magic[i - 1])
                } else {
                    u32::MAX
                };
            } else {
                i -= 1;
                // r = l + magic[i];
                ar = al;
                al = ask(l + magic[i] - magic[i - 1] - 1);
            }
        }
        if ar == u32::MAX {
            answer(al);
        } else {
            answer(al.max(ar));
        }
    }
}
