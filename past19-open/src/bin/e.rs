use proconio::input;

fn main() {
    input! {
        n: usize,
        mut s: proconio::marker::Bytes,
    }

    let mut i = 0;
    let mut prev = usize::MAX;
    while i < n - 1 {
        let si = s[i];
        if prev == usize::MAX && si == b'/' && s[i + 1] == b'*' {
            prev = i;
            i += 2;
            continue;
        }
        if prev != usize::MAX && si == b'*' && s[i + 1] == b'/' {
            s[prev..=i + 1].fill(0);
            prev = usize::MAX;
            i += 2;
            continue;
        }
        i += 1;
    }

    let ans = s.iter().copied().filter(|&c| c != 0).collect::<Vec<_>>();
    println!("{}", std::str::from_utf8(&ans).unwrap());
}
