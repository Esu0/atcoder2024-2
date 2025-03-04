use proconio::input;

fn main() {
    input! {
        n: usize,
        mut s: [String; n],
    }

    s.sort_unstable_by_key(|si| si.len());
    for si in &s {
        print!("{si}");
    }
    println!();
}
