use proconio::input;

fn main() {
    input! {
        s1: String,
        s2: String,
    }

    let m = if s1.as_str() == "sick" { 0 } else { 1 };
    let l = if s2.as_str() == "sick" { 0 } else { 1 };
    println!("{}", (m << 1) + l + 1);
}
