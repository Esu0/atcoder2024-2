use proconio::input;

fn main() {
    input! {
        n: usize,
        mut p: [usize; n],
    }

    let mut r = 1usize;
    let mut ans = vec![0; n];
    loop {
        let mx = p.iter().copied().max().unwrap();
        if mx == 0 {
            break;
        }
        let mut k = 0;
        for (i, pi) in p.iter_mut().enumerate() {
            if *pi == mx {
                ans[i] = r;
                *pi = 0;
                k += 1;
            }
        }
        r += k;
    }
    ans.iter().for_each(|&x| println!("{x}"));
}
