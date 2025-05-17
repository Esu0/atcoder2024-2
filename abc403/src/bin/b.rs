use proconio::input;

fn main() {
    input! {
        mut t: proconio::marker::Bytes,
        u: proconio::marker::Bytes,
    }

    let mut inds = [usize::MAX; 4];
    let mut ln = 0;
    for (i, &ti) in t.iter().enumerate() {
        if ti == b'?' {
            inds[ln] = i;
            ln += 1;
        }
    }
    let rng = b'a'..=b'z';
    for c1 in rng.clone() {
        for c2 in rng.clone() {
            for c3 in rng.clone() {
                for c4 in rng.clone() {
                    let chars = [c1, c2, c3, c4];
                    for (j, &i) in inds.iter().enumerate() {
                        t[i] = chars[j];
                    }
                    for i in 0..t.len() - u.len() + 1 {
                        if t[i..i + u.len()] == u[..] {
                            println!("Yes");
                            return;
                        }
                    }
                }
            }
        }
    }
    println!("No");
}
