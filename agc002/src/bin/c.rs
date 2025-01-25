use proconio::input;

fn main() {
    input! {
        n: usize,
        l: u32,
        a: [u32; n],
    }
    let mut prev = a[0];
    let mut k= usize::MAX;
    for (i, &ai) in a.iter().enumerate().skip(1) {
        if prev + ai >= l {
            k = i;
            break;
        }
        prev = ai;
    }
    if k == usize::MAX {
        println!("Impossible");
        return;
    }
    println!("Possible");
    for j in 1..k {
        println!("{j}");
    }
    for j in (k + 1..n).rev() {
        println!("{j}");
    }
    println!("{k}");
}
