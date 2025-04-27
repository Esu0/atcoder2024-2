use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut boxes = Vec::with_capacity(n);
    for _ in 0..n {
        input! { ci: usize }
        boxes.push(std::iter::once(ci).collect::<std::collections::HashSet<_>>());
    }

    for _ in 0..q {
        input! {
            a: usize,
            b: usize,
        }

        let mut ba = std::mem::take(&mut boxes[a - 1]);
        let mut bb = std::mem::take(&mut boxes[b - 1]);
        if ba.len() > bb.len() {
            std::mem::swap(&mut ba, &mut bb);
        }
        bb.extend(ba);
        boxes[b - 1] = bb;
        println!("{}", boxes[b - 1].len());
    }
}
