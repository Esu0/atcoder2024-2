use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [u32; n],
        b: [u32; m],
    }

    let mut idxs1 = Vec::with_capacity(m);
    let mut j = 0;
    for (i, &ai) in a.iter().enumerate() {
        if ai == b[j] {
            j += 1;
            idxs1.push(i);
        }
        if j == m {
            break;
        }
    }
    if idxs1.len() != m {
        println!("No");
        return;
    }

    let mut idxs2 = vec![usize::MAX; m];
    let mut j = m;
    for (i, &ai) in a.iter().enumerate().rev() {
        if ai == b[j - 1] {
            j -= 1;
            idxs2[j] = i;
        }
        if j == 0 {
            break;
        }
    }
    assert_eq!(idxs2.len(), m);
    if idxs1 == idxs2 {
        println!("No");
    } else {
        println!("Yes");
    }
}
