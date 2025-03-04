use proconio::input;

fn fill2(n: usize, g: &mut [Vec<u8>], a: usize, c: u8) {
    for i in a..n - a {
        for j in a..n-a {
            g[i][j] = c;
        }
    }
}

fn main() {
    input! {
        n: usize,
    }

    let mut g = vec![vec![0; n]; n];

    for x in 0..n + 1 / 2 {
        let c = if x % 2 == 0 {
            b'#'
        } else {
            b'.'
        };
        fill2(n, &mut g, x, c);
    }
    g.iter().for_each(|row| {
        println!("{}", std::str::from_utf8(row).unwrap())
    })
}
