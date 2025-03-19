use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut s1 = vec![0; n + 1];
    let mut s2 = vec![0; n + 1];
    let mut l1 = 0usize;
    let mut l2 = 0usize;
    for &ai in &a {
        if s2[ai] == 0 {
            l2 += 1;
        }
        s2[ai] += 1;
    }

    let mut ans = 0;
    for &ai in &a {
        ans = ans.max(l1 + l2);
        if s1[ai] == 0 {
            l1 += 1;
        }
        s1[ai] += 1;
        s2[ai] -= 1;
        if s2[ai] == 0 {
            l2 -= 1;
        }
    }

    println!("{ans}");
}
