use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
        q: [usize; n],
    }

    let mut iq = vec![usize::MAX; n];
    for (i, &qi) in q.iter().enumerate() {
        iq[qi - 1] = i;
    }

    for i in 0..n {
        print!("{} ", q[p[iq[i]] - 1]);
    }
    println!();
}
