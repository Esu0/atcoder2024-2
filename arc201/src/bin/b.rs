use proconio::input;

fn main() {
    input! { t: usize }

    let mut goods = std::array::from_fn::<_, 60, _>(|_| Vec::new());
    let mut buf1 = Vec::with_capacity(4<<17);
    let mut buf2 = Vec::with_capacity(4<<17);
    for _ in 0..t {
        input! {
            n: usize,
            w: u64,
        }
        goods.iter_mut().for_each(Vec::clear);
        buf1.clear();
        buf2.clear();
        for _ in 0..n {
            input! { x: usize, y: u64 }
            goods[x].push(y);
        }
        let mut ans = 0;
        for (i, gi) in goods.iter().enumerate() {
            buf2.clear();
            while let Some(h) = buf1.pop() {
                let v = h + buf1.pop().unwrap_or_default();
                buf2.push(v);
            }
            std::mem::swap(&mut buf1, &mut buf2);
            buf1.extend_from_slice(gi);
            buf1.sort_unstable();

            if (w >> i) & 1 != 0 {
                ans += buf1.pop().unwrap_or_default();
            }
        }
        println!("{ans}");
    }
}
