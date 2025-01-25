use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
        m: usize,
    }

    let mut mat = [[1i8, 0], [0, 1]];
    let prod = |m1: [[i8; 2]; 2], m2: [[i8; 2]; 2]| {
        [
            [m1[0][0] * m2[0][0] + m1[0][1] * m2[1][0], m1[0][0] * m2[0][1] + m1[0][1] * m2[1][1]],
            [m1[1][0] * m2[0][0] + m1[1][1] * m2[1][0], m1[1][0] * m2[0][1] + m1[1][1] * m2[1][1]],
        ]
    };
    let mut p = (0, 0);
    let mut ops = vec![(mat, p)];
    for _ in 0..m {
        input! { t: u8 }
        match t {
            2 => {
                mat = prod([[0, -1], [1, 0]], mat);
                (p.0, p.1) = (-p.1, p.0);
                ops.push((mat, p));
            }
            1=> {
                mat = prod([[0, 1], [-1, 0]], mat);
                (p.0, p.1) = (p.1, -p.0);
                ops.push((mat, p));
            }
            3 => {
                input! { q: i64 }
                mat = prod([[-1, 0], [0, 1]], mat);
                p.0 = -p.0;
                p.0 += q * 2;
                ops.push((mat, p));
            }
            4 => {
                input! { q: i64 }
                mat = prod([[1, 0], [0, -1]], mat);
                p.1 = -p.1;
                p.1 += q * 2;
                ops.push((mat, p));
            }
            _ => unreachable!(),
        }
    }
    input! { q: usize }
    for _ in 0..q {
        input! {
            a: usize,
            b: usize,
        }
        let mut x = xy[b - 1];
        let (m, p) = ops[a];
        x = (m[0][0] as i64 * x.0 + m[0][1] as i64 * x.1, m[1][0] as i64 * x.0 + m[1][1] as i64 * x.1);
        x.0 += p.0;
        x.1 += p.1;
        println!("{} {}", x.0, x.1);
    }
}
