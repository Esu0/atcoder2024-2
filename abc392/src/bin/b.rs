use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
    }
    let mut ans = vec![];
    for i in 1..=n {
        if !a.contains(&i) {
            ans.push(i);
        }
    }
    println!("{}", ans.len());
    ans.iter().for_each(|&ai| print!("{ai} "));
    println!();
}
