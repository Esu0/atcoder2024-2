use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }
    let sum = a.iter().copied().sum::<u64>();
    if sum % 10 != 0 {
        println!("No");
        return;
    }

    let target = sum / 10;
    let a = a.iter().chain(&a).copied().collect::<Vec<_>>();
    let mut s = 0;
    let mut j = 0;
    for &ai in a.iter() {
        while j < 2 * n && s < target {
            s += a[j];
            j += 1;
        }
        if s == target {
            println!("Yes");
            return;
        }
        s -= ai;
    }
    println!("No");
}
