use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [u32; n],
    }

    a.sort_unstable();
    for (i, &ai) in a.iter().enumerate() {
        print!("{ai}");
        if i == n - 1 {
            println!();
        } else {
            print!(" ");
        }
    }
}
