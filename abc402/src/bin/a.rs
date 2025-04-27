use proconio::input;

fn main() {
    input! {
        s: proconio::marker::Bytes,
    }

    let ans = s.iter().copied().filter(|&c| c.is_ascii_uppercase()).collect::<Vec<_>>();
    println!("{}", std::str::from_utf8(&ans).unwrap());
}
