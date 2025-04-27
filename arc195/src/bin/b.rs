use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i32; n],
        b: [i32; n],
    }

    a.sort_unstable();
    let last = *a.last().unwrap();
    if last == -1 {
        println!("Yes");
        return;
    }

    
}
