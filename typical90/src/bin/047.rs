use proconio::{input, marker};

const M: u64 = (1 << 61) - 1;

fn prodm(a: u64, b: u64) -> u64 {
    let a0 = a >> 31;
    let b0 = b >> 31;
    let a1 = a & 0x7fffffff;
    let b1 = b & 0x7fffffff;
    let m = a0 * b1 + a1 * b0;
    (2 * a0 * b0 + (m >> 30) + ((m & 0x3fffffff) << 31) + a1 * b1) % M
}

fn subm(a: u64, b: u64) -> u64 {
    let t = a + M - b;
    if t >= M {
        t - M
    } else {
        t
    }
}

fn main() {
    input! {
        n: usize,
        s: marker::Bytes,
        t: marker::Bytes,
    }
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let x1 = loop {
        let tmp = rng.gen_range(2..M);
        if M % tmp != 0 {
            break tmp;
        }
    };
    let x2 = loop {
        let tmp = rng.gen_range(2..M);
        if M % tmp != 0 {
            break tmp;
        }
    };

    let pow1 = {
        let mut v = vec![0; n + 1];
        v[0] = 1;
        for i in 0..n {
            v[i + 1] = prodm(v[i], x1);
        }
        v
    };
    let pow2 = {
        let mut v = vec![0; n + 1];
        v[0] = 1;
        for i in 0..n {
            v[i + 1] = prodm(v[i], x2);
        }
        v
    };

    let hash_s1 = {
        let mut v = vec![0; n + 1];
        for i in 0..n {
            v[i + 1] = (prodm(v[i], x1) + s[i] as u64) % M;
        }
        v
    };
    let hash_s2 = {
        let mut v = vec![0; n + 1];
        for i in 0..n {
            v[i + 1] = (prodm(v[i], x2) + s[i] as u64) % M;
        }
        v
    };

    let get_hash_s = |l: usize, r: usize| {
        [
            subm(hash_s1[r], prodm(hash_s1[l], pow1[r - l])),
            subm(hash_s2[r], prodm(hash_s2[l], pow2[r - l])),
        ]
    };

    fn change_color(c1: u8, c2: u8) -> u8 {
        match (c1, c2) {
            (b'R', b'R') | (b'G', b'B') | (b'B', b'G') => b'R',
            (b'G', b'G') | (b'R', b'B') | (b'B', b'R') => b'G',
            (b'B', b'B') | (b'R', b'G') | (b'G', b'R') => b'B',
            _ => panic!("unexpected color"),
        }
    }

    let mut ans = 0;
    let cols = [b'R', b'G', b'B'];
    let mut hash2 = [[0; 2]; 3];
    for i in 0..n {
        for j in 0..3 {
            hash2[j][0] = (prodm(hash2[j][0], x1) + change_color(cols[j], t[i]) as u64) % M;
            hash2[j][1] = (prodm(hash2[j][1], x2) + change_color(cols[j], t[i]) as u64) % M;
        }
        if hash2.contains(&get_hash_s(n - i - 1, n)) {
            ans += 1;
        }
    }
    hash2 = [[0; 2]; 3];
    for i in 0..n - 1 {
        for j in 0..3 {
            hash2[j][0] =
                (hash2[j][0] + prodm(change_color(cols[j], t[n - i - 1]) as u64, pow1[i])) % M;
            hash2[j][1] =
                (hash2[j][1] + prodm(change_color(cols[j], t[n - i - 1]) as u64, pow2[i])) % M;
        }
        if hash2.contains(&get_hash_s(0, i + 1)) {
            ans += 1;
        }
    }

    // {
    //     // solve naive
    //     let mut grid = vec![vec![0; n]; n];
    //     for i in 0..n {
    //         for j in 0..n {
    //             grid[i][j] = change_color(s[i], t[j]);
    //         }
    //     }
    //     let mut count = 0;
    //     for i in 0..n {
    //         let t = grid[0][i];
    //         if (i + 1..n).enumerate().all(|(j, k)| grid[j + 1][k] == t) {
    //             count += 1;
    //         }
    //     }
    //     for i in 1..n {
    //         let t = grid[i][0];
    //         if (i + 1..n).enumerate().all(|(j, k)| grid[k][j + 1] == t) {
    //             count += 1;
    //         }
    //     }
    //     assert_eq!(count, ans);
    //     eprintln!("{count}");
    // }

    println!("{ans}");
}
