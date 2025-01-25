use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
        c: [usize; n],
    }
    let mut ac = [0u64; 46];
    let mut bc = [0; 46];
    let mut cc = [0; 46];
    for (&ai, (&bi, &ci)) in a.iter().zip(b.iter().zip(&c)) {
        ac[ai % 46] += 1;
        bc[bi % 46] += 1;
        cc[ci % 46] += 1;
    }
    let mut ans = 0;
    for i in 0..46 {
        for j in 0..46 {
            ans += ac[i] * bc[j] * cc[(46 * 2 - i - j) % 46];
        }
    }
    println!("{ans}");
}
