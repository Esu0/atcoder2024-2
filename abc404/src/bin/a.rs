use proconio::input;

fn main() {
    input! {
        s: proconio::marker::Bytes,
    }

    let set = s.iter().copied().collect::<std::collections::HashSet<_>>();
    for i in b'a'..=b'z' {
        if !set.contains(&i) {
            println!("{}", i as char);
            return;
        }
    }
}
