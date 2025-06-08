use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut stack = Vec::with_capacity(n + 1);
    stack.push(n);
    let mut ans = 0;
    for i in (0..n).rev() {
        stack.push(i);
        for j in 1.. {
            let top = *stack.last().unwrap();
            if top < n && a[top] == j {
                stack.pop();
            } else {
                break;
            }
        }
        ans += *stack.last().unwrap() - i;
    }
    println!("{ans}");
}
