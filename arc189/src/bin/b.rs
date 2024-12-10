use proconio::input;

fn main() {
    input! {
        n: usize,
        x: [usize; n],
    }
    let mut y = x.clone();
    for i in (1..n).rev() {
        y[i] -= y[i - 1];
    }
    let mut odd = y[1..].iter().copied().step_by(2).collect::<Vec<_>>();
    let mut even = y[2..].iter().copied().step_by(2).collect::<Vec<_>>();
    odd.sort_unstable();
    even.sort_unstable();
    let mut j = 0;
    let mut k = 0;
    for i in 1..n {
        if i % 2 != 0 {
            y[i] = odd[j];
            j += 1;
        } else {
            y[i] = even[k];
            k += 1;
        }
    }
    let mut sum = y[0];
    let mut ans = sum;
    for i in 1..n {
        sum += y[i];
        ans += sum;
    }
    println!("{ans}");
}
