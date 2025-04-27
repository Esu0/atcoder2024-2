use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut logined = false;
    let mut ans = 0;

    for _ in 0..n {
        input! { s: String }
        if s == "login" {
            logined = true;
        } else if s == "logout" {
            logined = false;
        } else if s == "private" {
            if !logined {
                ans += 1;
            }
        }
    }
    println!("{ans}");
}
