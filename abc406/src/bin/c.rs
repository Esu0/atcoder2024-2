use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }

    let mut i = 1;
    let mut ans = 0;
    let mut arr = vec![];
    loop {
        let mut cnt = 0usize;
        while i < n && p[i] > p[i - 1] {
            i += 1;
            cnt += 1;
        }
        arr.push(cnt);

        if i >= n {
            break;
        }

        while i < n && p[i] < p[i - 1] {
            i += 1;
        }

        if i >= n {
            break;
        }
    }
    // eprintln!("{arr:?}");
    for i in 1..arr.len() {
        ans += arr[i] * arr[i - 1];
    }
    println!("{ans}");

}
