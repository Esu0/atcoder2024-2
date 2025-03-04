use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut locate = (0..n).collect::<Vec<_>>();
    let mut map = locate.clone();
    let mut map_rev = locate.clone();
    for _ in 0..q {
        input! { t: u8 }
        if t == 1 {
            input! { a: usize, b: usize }
            locate[a - 1] = map_rev[b - 1];
        } else if t == 2 {
            input! { a: usize, b: usize }
            let ia = map_rev[a - 1];
            let ib = map_rev[b - 1];
            map[ia] = b - 1;
            map[ib] = a - 1;
            map_rev[a - 1] = ib;
            map_rev[b - 1] = ia;
        } else {
            input! { a: usize }
            let ans = map[locate[a - 1]];
            println!("{}", ans + 1);
        }
    }
}
