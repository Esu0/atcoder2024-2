use proconio::input;

fn to_left(c: u8) -> u8 {
    if c == b']' {
        b'['
    } else if c == b')' {
        b'('
    } else if c == b'>' {
        b'<'
    } else {
        unreachable!()
    }
}

fn main() {
    input! {
        s: proconio::marker::Bytes,
    }

    let mut stack = vec![];
    for &si in &s {
        if [b'[', b'<', b'('].contains(&si) {
            stack.push(si);
        } else if stack.pop() != Some(to_left(si)) {
            println!("No");
            return;
        }
    }
    if stack.is_empty() {
        println!("Yes");
    } else {
        println!("No");
    }
}
