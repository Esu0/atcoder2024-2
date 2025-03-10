use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }

    let mut imos = vec![0i64; n];

    let mut pos = vec![usize::MAX; n];
    for (i, &pi) in p.iter().enumerate() {
        pos[pi - 1] = i;
    }

    
}
