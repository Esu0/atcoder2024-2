use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut ans = Vec::with_capacity(q);
    let mut boxes = vec![0usize; n];
    for _ in 0..q {
        input! { x: usize }
        if x > 0 {
            boxes[x - 1] += 1;
            ans.push(x);
        } else {
            let min = boxes.iter().copied().min().unwrap();
            for i in 0..n {
                if boxes[i] == min {
                    ans.push(i + 1);
                    boxes[i] += 1;
                    break;
                }
            }
        }
    }

    print!("{}", ans[0]);
    for &ansi in &ans[1..] {
        print!(" {}", ansi);
    }
    println!();
}
