use proconio::input;

fn main() {
    input! {
        n: usize,
        cl: [(char, usize); n],
    }
    let mut ans = vec![];
    for &(ci, li) in &cl {
        for _ in 0..li {
            if ans.len() >= 100 {
                println!("Too Long");
                return;
            }
            ans.push(ci as u8);
        }
    }
    println!("{}", std::str::from_utf8(&ans).unwrap());
}
