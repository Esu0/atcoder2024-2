use proconio::input;
// use ac_library::LazySegtree;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut pos = vec![0; n];
    let mut ans = 0;
    let mut s = 0;
    for i in 0..n {
        let f = pos[a[i] - 1];
        let mut f1 = if a[i] == 1 { 0 } else { pos[a[i] - 2] };
        let mut f2 = if a[i] == n { 0 } else { pos[a[i]] };
        if f1 > f2 {
            (f1, f2) = (f2, f1);
        }
        if f > 0 && f > f2 {
            s += i - f + 1;
        } else if (f1..=f2).contains(&f) {

        } else {
            s += (i - f2 + 1) - f1;
            pos[a[i] - 1] = i + 1;
        }
        ans += s;
    }
    println!("{ans}");
}
