use proconio::input;

fn main() {
    input! {
        n: usize,
        s: proconio::marker::Bytes,
    }

    let mut set = [0, 0];
    let mut cnt = 0;
    let mut j = 0;
    let mut ans = 0;
    for &si in &s {
        while j < n && cnt != 2 {
            let ind = if s[j] == b'o' { 0 } else { 1 };
            if set[ind] == 0 {
                cnt += 1;
            }
            set[ind] += 1;
            j += 1;
        }
        if cnt != 2 {
            break;
        }
        // eprintln!("{j}");
        let ind = if si == b'o' { 0 } else { 1 };
        set[ind] -= 1;
        if set[ind] == 0 {
            cnt -= 1;
        }
        ans += n - j + 1;
    }
    println!("{ans}");
}
