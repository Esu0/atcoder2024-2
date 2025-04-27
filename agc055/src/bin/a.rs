
use proconio::input;

fn main() {
    input! {
        n: usize,
        s: proconio::marker::Bytes,
    }

    let mut idx_sets = std::array::from_fn::<_, 3, _>(|_| std::array::from_fn::<_, 3, _>(|_| Vec::new()));

    for j in 0..3 {
        for i in 0..n {
            idx_sets[j][(s[i + j * n] - b'A') as usize].push(i + j * n);
        }
    }

    // eprintln!("{idx_sets:?}");
    let mut x = b'1';
    let mut ans = vec![b'x'; 3 * n];
    for i in 0..3 {
        for j in 0..3 {
            if i == j {
                continue;
            }
            for k in 0..3 {
                if i == k || j == k {
                    continue;
                }
                let cnt = idx_sets[0][i].len().min(idx_sets[1][j].len()).min(idx_sets[2][k].len());
                for _ in 0..cnt {
                    ans[idx_sets[0][i].pop().unwrap()] = x;
                }
                for _ in 0..cnt {
                    ans[idx_sets[1][j].pop().unwrap()] = x;
                }
                for _ in 0..cnt {
                    ans[idx_sets[2][k].pop().unwrap()] = x;
                }
                x += 1;
            }
        }
    }
    println!("{}", std::str::from_utf8(&ans).unwrap());
}
