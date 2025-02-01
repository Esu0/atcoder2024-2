use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    // 各鳩がいる巣の番号
    let mut pos = (0..n).collect::<Vec<_>>();
    // let mut pigeon = pos.clone();
    let mut count = (0..n).map(|_| 1u32).collect::<Vec<_>>();
    let mut ans = 0;

    for _ in 0..q {
        input! { t: u8 }

        if t == 1 {
            input! { p: usize, h: usize }
            let p = p - 1;
            let h = h - 1;
            let posp = pos[p];
            if count[posp] == 2 {
                ans -= 1;
            }
            count[posp] -= 1;
            pos[p] = h;
            if count[h] == 1 {
                ans += 1;
            }
            count[h] += 1;
        } else {
            println!("{ans}");
        }
    }
}
